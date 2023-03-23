use color_eyre::eyre::eyre;
use color_eyre::Result;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::{
    line_ending, multispace1, newline, not_line_ending, one_of, space1,
};
use nom::combinator::{not, opt};
use nom::multi::{many0, many1};
use nom::sequence::terminated;
// use nom::IResult;
use once_cell::unsync::OnceCell;
use std::collections::HashSet;

pub type IResult<I, O> = nom::IResult<I, O, nom_supreme::error::ErrorTree<I>>;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use yare::parameterized;

    #[test]
    fn parse_ls_output_returns_files_in_directory() {
        // Given
        let input = indoc! {
            "$ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            "
        };

        let expected_files = vec![
            File {
                name: "b.txt".to_owned(),
                size: 14848514,
            },
            File {
                name: "c.dat".to_owned(),
                size: 8504156,
            },
        ];

        // When
        let (res, file_list) = parse_ls_output(input).unwrap();

        // Then
        assert_eq!(file_list, expected_files);
        assert!(res.is_empty(), r#"Res not empty! Leftovers: "{res}""#);
    }

    #[test]
    fn parse_tree_returns_directory_tree() {
        // Given
        let input = indoc! {
            "$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
            "
        };

        let expected_tree = Directory::new(
            "/",
            vec![
                File {
                    name: "b.txt".to_owned(),
                    size: 14848514,
                },
                File {
                    name: "c.dat".to_owned(),
                    size: 8504156,
                },
            ],
            vec![
                Directory::new(
                    "a",
                    vec![
                        File {
                            name: "f".to_owned(),
                            size: 29116,
                        },
                        File {
                            name: "g".to_owned(),
                            size: 2557,
                        },
                        File {
                            name: "h.lst".to_owned(),
                            size: 62596,
                        },
                    ],
                    vec![Directory::new(
                        "e",
                        vec![File {
                            name: "i".to_owned(),
                            size: 584,
                        }],
                        vec![],
                    )],
                ),
                Directory::new(
                    "d",
                    vec![
                        File {
                            name: "j".to_owned(),
                            size: 4060174,
                        },
                        File {
                            name: "d.log".to_owned(),
                            size: 8033020,
                        },
                        File {
                            name: "d.ext".to_owned(),
                            size: 5626152,
                        },
                        File {
                            name: "k".to_owned(),
                            size: 7214296,
                        },
                    ],
                    vec![],
                ),
            ],
        );

        // When
        let outcome = parse_tree(input);

        // Then
        match outcome {
            Ok((res, actual_tree)) => {
                assert!(res.is_empty(), r#"Res not empty! Leftovers: "{res}""#);
                assert_eq!(actual_tree, expected_tree);
            }
            Err(e) => {
                println!("{e:#?}");
                panic!("Parser failure!");
            }
        }
    }

    #[test]
    fn directory_size_returns_sum_of_file_sizes_plus_sum_of_subdirectory_sizes() {
        // Given
        let tree = Directory::new(
            "/",
            vec![
                File {
                    name: "b.txt".to_owned(),
                    size: 14848514,
                },
                File {
                    name: "c.dat".to_owned(),
                    size: 8504156,
                },
            ],
            vec![
                Directory::new(
                    "a",
                    vec![
                        File {
                            name: "f".to_owned(),
                            size: 29116,
                        },
                        File {
                            name: "g".to_owned(),
                            size: 2557,
                        },
                        File {
                            name: "h.lst".to_owned(),
                            size: 62596,
                        },
                    ],
                    vec![Directory::new(
                        "e",
                        vec![File {
                            name: "i".to_owned(),
                            size: 584,
                        }],
                        vec![],
                    )],
                ),
                Directory::new(
                    "d",
                    vec![
                        File {
                            name: "j".to_owned(),
                            size: 4060174,
                        },
                        File {
                            name: "d.log".to_owned(),
                            size: 8033020,
                        },
                        File {
                            name: "d.ext".to_owned(),
                            size: 5626152,
                        },
                        File {
                            name: "k".to_owned(),
                            size: 7214296,
                        },
                    ],
                    vec![],
                ),
            ],
        );

        // When
        let actual_size = tree.size();

        // Then
        assert_eq!(actual_size, 48381165);
    }
}

pub fn solve_part_1(input_data: &str) -> String {
    let outcome = parse_tree(input_data);

    let tree = match outcome {
        Ok((res, tree)) => {
            assert!(res.is_empty(), r#"Res not empty! Leftovers: "{res}""#);

            tree
        }
        Err(e) => {
            println!("{e:#?}");
            panic!("Parser failure!");
        }
    };

    let mut size_accumulator = 0u64;
    let mut collect_sizes = |d: &Directory| {
        let size = d.size();
        if (size < 100_000) {
            size_accumulator += size;
        }
    };

    tree.walk_apply(&mut collect_sizes);

    return format!("{size_accumulator}");
}

pub fn solve_part_2(input_data: &str) -> String {
    todo!()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
    size: OnceCell<u64>,
}

impl Directory {
    fn new(name: &str, files: Vec<File>, subdirectories: Vec<Directory>) -> Self {
        Self {
            name: name.to_owned(),
            files,
            subdirectories,
            size: OnceCell::new(),
        }
    }

    fn size(&self) -> u64 {
        *self.size.get_or_init(|| {
            let total_file_size: u64 = self.files.iter().map(|f| f.size).sum();
            let total_subdir_size: u64 = self.subdirectories.iter().map(|d| d.size()).sum();

            total_file_size + total_subdir_size
        })
    }

    fn walk_apply<F: FnMut(&Self)>(&self, f: &mut F) {
        (*f)(self);

        for d in self.subdirectories.iter() {
            d.walk_apply(f);
        }
    }
}

fn take_rest_of_line(i: &str) -> IResult<&str, &str> {
    let (res, rest_of_line) = not_line_ending(i)?;
    let (res, _) = newline(res)?;

    Ok((res, rest_of_line))
}

fn parse_ls_output(i: &str) -> IResult<&str, Vec<File>> {
    fn parse_file(i: &str) -> IResult<&str, Option<File>> {
        let (res, size) = nom::character::complete::u64(i)?;
        let (res, _) = space1(res)?;
        let (res, name) = take_rest_of_line(res)?;

        Ok((
            res,
            Some(File {
                name: name.to_owned(),
                size,
            }),
        ))
    }

    fn parse_dir(i: &str) -> IResult<&str, Option<File>> {
        let (res, _) = tag("dir ")(i)?;
        let (res, _name) = take_rest_of_line(res)?;

        Ok((res, None))
    }

    let (res, _) = tag("$ ls\n")(i)?;
    let (res, contents) = many1(alt((parse_file, parse_dir)))(res)?;

    let files: Vec<_> = contents.into_iter().flatten().collect();

    Ok((res, files))
}

fn parse_tree(i: &str) -> IResult<&str, Directory> {
    fn parse_cd(i: &str) -> IResult<&str, &str> {
        let _ = not(parse_cd_dot_dot)(i)?;
        let (res, _) = tag("$ cd ")(i)?;
        let (res, dir_name) = take_rest_of_line(res)?;

        Ok((res, dir_name))
    }

    fn parse_cd_dot_dot(i: &str) -> IResult<&str, ()> {
        let (res, _) = terminated(tag("$ cd .."), newline)(i)?;
        Ok((res, ()))
    }

    let (res, dir_name) = parse_cd(i)?;
    let (res, files) = parse_ls_output(res)?;

    let (res, subdirectories) = many0(parse_tree)(res)?;

    let (res, _) = opt(parse_cd_dot_dot)(res)?;

    Ok((res, Directory::new(dir_name, files, subdirectories)))
}
