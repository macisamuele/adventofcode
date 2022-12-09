use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::iter::Peekable;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum LsOutput {
    Dir,
    File { size: usize },
}
impl FromStr for LsOutput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(_name) = scan_fmt!(s, "dir {}", String) {
            Ok(Self::Dir)
        } else {
            let (size, _name) = scan_fmt!(s, "{} {}", usize, String)?;
            Ok(Self::File { size })
        }
    }
}

#[derive(Debug)]
enum Cmd {
    CD { cwd: PathBuf },
    LS { cwd: PathBuf, output: Vec<LsOutput> },
}

impl Cmd {
    fn from_shell_content<'a, I: Iterator<Item = &'a str>>(
        lines: &mut Peekable<I>,
        cwd: Option<&Path>,
    ) -> anyhow::Result<Self> {
        let cmd_line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected to have at least one line"))?;
        if Self::is_cmd(cmd_line) {
            match cmd_line {
                v if v.starts_with("$ cd ") => {
                    let new_path = v.replace("$ cd ", "");
                    if let Some(cwd) = cwd {
                        if new_path == ".." {
                            Ok(Self::CD {
                                cwd: cwd
                                    .parent()
                                    .ok_or_else(|| {
                                        anyhow::anyhow!(
                                            "{} is expected to have a parent",
                                            cwd.display()
                                        )
                                    })?
                                    .to_path_buf(),
                            })
                        } else {
                            let mut cwd_path_buf = cwd.to_path_buf();
                            cwd_path_buf.push(new_path);
                            Ok(Self::CD { cwd: cwd_path_buf })
                        }
                    } else {
                        Ok(Self::CD {
                            cwd: Path::new(&new_path).to_path_buf(),
                        })
                    }
                }
                v if v == "$ ls" => {
                    let mut output: Vec<LsOutput> = vec![];
                    while matches!(lines.peek(), Some(line) if !Self::is_cmd(line)) {
                        output.push(
                            lines
                                .next()
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "A line is expected to be present and not to be a command"
                                    )
                                })?
                                .parse()?,
                        );
                    }
                    Ok(Self::LS {
                        cwd: cwd
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "'ls' command expects cwd to be defined, received None"
                                )
                            })?
                            .to_path_buf(),
                        output,
                    })
                }
                _ => Err(anyhow::anyhow!(
                    "Unrecognised command '{cmd_line}'. Supported 'cd' and 'ls'"
                )),
            }
        } else {
            Err(anyhow::anyhow!(
                "Expected to find cmd (starting with $) found {cmd_line}"
            ))
        }
    }

    fn is_cmd(line: &str) -> bool {
        line.starts_with('$')
    }

    fn cwd(&self) -> &Path {
        match self {
            Self::CD { cwd, .. } | Self::LS { cwd, .. } => cwd,
        }
    }
}

struct Input {
    cmds: Vec<Cmd>,
}
impl TryFrom<&[String]> for Input {
    type Error = anyhow::Error;
    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut lines_iter = lines.iter().map(String::as_str).peekable();
        let mut cmds: Vec<Cmd> = vec![];
        while lines_iter.peek().is_some() {
            let cmd = Cmd::from_shell_content(&mut lines_iter, cmds.last().map(Cmd::cwd))?;
            cmds.push(cmd);
        }
        Ok(Self { cmds })
    }
}
impl Input {
    fn directory_to_size(&self) -> BTreeMap<&Path, usize> {
        let mut directory_to_size: BTreeMap<&Path, usize> =
            self.cmds.iter().map(|cmd| (cmd.cwd(), 0)).collect();

        for cmd in &self.cmds {
            if let Cmd::LS { cwd, output } = cmd {
                let direct_file_size: usize = output
                    .iter()
                    .map(|ls_output| match ls_output {
                        LsOutput::File { size, .. } => *size,
                        LsOutput::Dir { .. } => 0,
                    })
                    .sum();

                for (dir, size) in &mut directory_to_size {
                    if cwd.starts_with(dir) {
                        *size += direct_file_size;
                    }
                }
            }
        }
        directory_to_size
    }
}

fn part01(input: &Input) -> usize {
    input
        .directory_to_size()
        .values()
        .filter(|size| size < &&100_000)
        .sum()
}

fn part02(input: &Input) -> usize {
    let directory_to_size = input.directory_to_size();
    let space_needed_for_update = 30_000_000;
    let disk_size = 70_000_000;
    let space_to_release = directory_to_size[&Path::new("/")] + space_needed_for_update - disk_size;
    directory_to_size
        .values()
        .filter(|size| size >= &&space_to_release)
        .min()
        .copied()
        .unwrap_or(0)
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(&input_lines(INPUT)?[..])?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
