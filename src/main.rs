use ansi_term::{Colour, Style};
use std::io::{self, stdout, Write};
use std::env::current_dir;
use std::process::Command;
use git2::{Repository};

// let delimiter = "▙";
// for (i, seg) in segments.iter().enumerate() {
//     print!("{}",
//         Style::new().on(seg.color.foreground.unwrap()).paint(
//             format!(" {} {}",
//             seg.text,
//             if i+1 < segments.len() {
//                 seg.color.on(segments[i + 1].color.foreground.unwrap()).paint(delimiter).to_string()
//             } else {
//                 "".to_owned()
//             })
//         )
//     );

//     if i + 1 == segments.len() {
//         print!("{}", seg.color.paint(delimiter));
//     };
// }

struct Segment {
    text: String,
    color: Style
}

fn print_prompt(cwd: &String, current_repo: &Option<(String, String)>) {
    let (t_width, _) = term_size::dimensions()
        .expect("Could not get size of terminal.");
    let path_str = format!(
        "~/{}",
        if cfg!(target_os = "windows") {
            cwd.split("\\").last().unwrap()
        } else {
            cwd.split("/").last().unwrap()
        }
    );
    let mut segments = vec![
        Segment {
            text: path_str,
            color: Colour::Green.blink()
        },
        // Segment {
        //     text: "World!".to_owned(),
        //     color: Colour::Blue.normal()
        // }
    ];

    if current_repo.is_some() {
        let gitrepo = current_repo.clone().unwrap();

        segments.push(Segment {
            text: format!("{}/{}", gitrepo.0, gitrepo.1),
            color: Colour::Blue.bold()
        });
    }
    
    let num_of_extra_chars = 2;
    let padding_amount = 1;
    let margin_left = 2;
    let line_color = Colour::Fixed(245);

    print!("{}", line_color.paint("╭─"));
    for (i, seg) in segments.iter().enumerate() {
        print!("{}", Style::new().on(seg.color.foreground.unwrap()).paint(format!(" {} ", &seg.text)));

        if i != segments.len() -1 {
            print!("{}", line_color.paint("─".repeat(padding_amount)));
        }
    }

    let ps1_length: usize = num_of_extra_chars * segments.len()
        + padding_amount * (segments.len() - 1)
        + segments.iter().map(|seg| seg.text.len() as usize).fold(0, |a, b| a + b)
        + margin_left;

    println!("{}", line_color.paint("─".repeat(t_width - ps1_length)));
    print!("{}", line_color.paint("╰╴ "));
    let _ = stdout().flush();
}

fn get_repo_at_path(path: &String) -> Option<(String, String)> {
    let repo_result = Repository::discover(path);
    if repo_result.is_err() {
        return None
    }
    let repo = repo_result.unwrap();

    for remote in repo.remotes().unwrap().iter() {
        let remote_info = repo.find_remote(remote.unwrap()).unwrap();
        let remote_url = remote_info.url()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>();

        let (repo_name, repo_owner) = (
            remote_url[remote_url.len() - 1].strip_suffix(".git").expect("Repo url didnt have '.git' ??"),
            remote_url[remote_url.len() - 2]
        );

        return Some((repo_owner.to_owned(), repo_name.to_owned()));
    }

    return None;
}

fn main() {
    let cwd = current_dir().unwrap().as_path().to_str().unwrap().to_owned();
    let mut current_repo = get_repo_at_path(&cwd);
    let _ = ansi_term::enable_ansi_support();
    
    println!("CWD: {}, REPO: {}", cwd, match current_repo.clone() {
        Some(repo) => repo.0,
        None => "No repo found".to_owned()
    });
    loop {
        print_prompt(&cwd, &current_repo);

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input.");

        input = input.trim().to_owned();

        let mut proc = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(["/C".to_owned(), input])
                    .spawn()
                    .expect("Failed to run command")
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg(input)
                    .spawn()
                    .expect("Failed to run command")
        };

        proc.wait()
            .expect("Could not wait for process.");
    }
}
