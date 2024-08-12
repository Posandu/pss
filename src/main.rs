use std::char;
#[derive(Debug, Clone)]
enum ItemType<'a> {
    File(File<'a>),
    Dir(Dir<'a>),
    Root(Dir<'a>),
}

#[derive(Debug, Clone)]
struct MetaData<'a> {
    name: &'a str,
}

#[derive(Debug, Clone)]
struct File<'a> {
    contents: &'a [u8],
    meta: MetaData<'a>,
}

#[derive(Debug, Clone)]
struct Dir<'a> {
    children: Vec<ItemType<'a>>,
    meta: MetaData<'a>,
}

fn main() {
    debug_assert_eq!(is_valid_name(" - - - -"), false);
    debug_assert_eq!(is_valid_name("%%ripbozo"), false);
    debug_assert_eq!(is_valid_name("aaa$%#"), false);
    debug_assert_eq!(is_valid_name("aaa"), true);
    debug_assert_eq!(is_valid_name("111"), true);

    debug_assert_eq!(is_valid_path("/h"), Err("Invalid path".to_string()));
    debug_assert_eq!(is_valid_path("h//"), Err("Invalid path".to_string()));
    debug_assert_eq!(is_valid_path("h/f."), Err("Invalid path".to_string()));
    debug_assert_eq!(is_valid_path("h"), Ok(()));
    debug_assert_eq!(is_valid_path("h/f.a"), Ok(()));

    let mut pss = create_fs();

    create_dir(&mut pss, "folder-1").unwrap();
    create_dir(&mut pss, "folder-1/test").unwrap();
    create_dir(&mut pss, "folder-2/hello").unwrap();
    create_dir(&mut pss, "folder-2/hellow/collection").unwrap();

    let gb = "y dsfs 54005 🥰🥰 🤦‍♂️".repeat(1_000_000_00);

    create_file(
        &mut pss,
        "folder-2/readme.md",
        "this is so cool 🤦‍♂️".as_bytes(),
    )
    .unwrap();
    create_file(
        &mut pss,
        "folder-2/hellow/collection/virus.exe",
        "this is cool".as_bytes(),
    )
    .unwrap();
    create_file(&mut pss, "main-readme.md", gb.as_bytes()).unwrap();
    create_file(&mut pss, "main-readme2.md", "this is so cool".as_bytes()).unwrap();

    pretty_print(&pss, "".to_string());
}

fn pretty_print<'a>(fs: &ItemType<'a>, indent: String) {
    match fs {
        ItemType::Root(root) => {
            for item in &root.children {
                pretty_print(item, indent.clone());
            }
        }
        ItemType::Dir(dir) => {
            println!("{}{}/", indent, dir.meta.name);
            for item in &dir.children {
                pretty_print(item, indent.clone() + "| ");
            }
        }
        ItemType::File(file) => {
            println!("{}{}", indent, file.meta.name);
        }
    }
}

fn create_fs<'a>() -> ItemType<'a> {
    ItemType::Root(Dir {
        meta: MetaData { name: "$$root" },
        children: vec![],
    })
}

fn is_valid_name<'a>(name: &'a str) -> bool {
    let chars = name.chars();
    let len = name.len();
    let max_len = 42;
    let min_len = 1;
    let mut is_ok = true;

    if len < min_len || len > max_len {
        return false;
    }

    for char in chars {
        if !(char.is_alphanumeric()) && char != '-' && char != '.' && char != '/' {
            is_ok = false;
        }
    }

    is_ok
}

fn is_valid_path<'a>(path: &'a str) -> Result<(), String> {
    let max_len = 256;
    let chars = path.as_bytes(); // using bytes cuz only ASCII is used
    let mut i = 0;

    if &chars.len() > &max_len {
        Some("Path too long".to_string());
    }

    loop {
        if chars[i] as char == '/' {
            if i == 0 {
                return Err("Invalid path".to_string());
            } else {
                let char_after = if i < path.len() {
                    chars[i + 1] as char
                } else {
                    '/'
                };
                let char_before = chars[i - 1] as char;

                if char_after == '/' || char_before == '/' {
                    return Err("Invalid path".to_string());
                }
            }
        } else if chars[i] as char == '.' {
            if i == 0 {
                return Err("Invalid path".to_string());
            } else {
                let char_after = if i < path.len() - 1 {
                    chars[i + 1] as char
                } else {
                    '.'
                };
                let char_before = chars[i - 1] as char;

                if char_after == '.'
                    || char_after == '/'
                    || char_before == '.'
                    || char_before == '/'
                {
                    return Err("Invalid path".to_string());
                }
            }
        }

        i += 1;

        if i == path.len() {
            break;
        }
    }

    Ok(())
}

fn create_dir<'a>(root: &mut ItemType<'a>, path: &'a str) -> Result<(), &'static str> {
    if !is_valid_name(path) {
        panic!("Invalid name.")
    }

    let navigateable = is_valid_path(path);

    match navigateable {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }

    let parts: Vec<&str> = path.split('/').collect();

    fn create_recursive<'a>(
        current: &mut ItemType<'a>,
        parts: &[&'a str],
    ) -> Result<(), &'static str> {
        match current {
            ItemType::Dir(dir) | ItemType::Root(dir) => {
                if let Some((first, rest)) = parts.split_first() {
                    if let Some(existing) = dir.children.iter_mut().find(|child| {
                        matches!(child, ItemType::Dir(d) | ItemType::Root(d) if &d.meta.name == first)
                    }) {
                        create_recursive(existing, rest)
                    } else {
                        let new_dir = ItemType::Dir(Dir {
                            children: Vec::new(),
                            meta: MetaData { name: &first },
                        });

                        dir.children.push(new_dir);

                        create_recursive(dir.children.last_mut().unwrap(), rest)
                    }
                } else {
                    Ok(())
                }
            }

            ItemType::File(_) => Err("Attempt to create folder inside file"),
        }
    }

    create_recursive(root, &parts)
}

fn create_file<'a>(
    root: &mut ItemType<'a>,
    path: &'a str,
    contents: &'a [u8],
) -> Result<(), String> {
    if !is_valid_name(path) {
        panic!("Invalid name.")
    }

    let navigateable = is_valid_path(path);

    match navigateable {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }

    let parts: Vec<&str> = path.split('/').collect();

    fn create_recursive<'a>(
        current: &mut ItemType<'a>,
        parts: &[&'a str],
        contents: &'a [u8],
    ) -> Result<(), String> {
        match current {
            ItemType::Dir(parent) | ItemType::Root(parent) => {
                if let Some((first, rest)) = parts.split_first() {
                    if rest.len() == 0 {
                        // directly creating the file in the root dir
                        parent.children.push(ItemType::File(File {
                            contents,
                            meta: MetaData { name: first },
                        }));

                        Ok(())
                    } else {
                        // subdir
                        if let Some(dir_exists) = parent.children.iter_mut().find(|i| match i {
                            ItemType::Dir(dir) | ItemType::Root(dir) => {
                                if &dir.meta.name == first {
                                    true
                                } else {
                                    false
                                }
                            }
                            ItemType::File(_) => false,
                        }) {
                            create_recursive(dir_exists, rest, &contents).unwrap();
                        } else {
                            return Err(format!("Directory {} doesn't exist", first));
                        }

                        return Ok(());
                    }
                } else {
                    Ok(())
                }
            }

            ItemType::File(_) => Err("Attempt to create file inside file".to_string()),
        }
    }

    create_recursive(root, &parts, &contents)
}
