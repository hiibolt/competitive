#[derive(Debug, Clone)]
enum DiskPartition {
    File(File),
    FreeSpace(FreeSpace)
}
#[derive(Debug, Clone)]
struct FreeSpace {
    size: usize
}
impl FreeSpace {
    fn render ( &self ) -> String {
        std::iter::once('.').cycle().take( self.size as usize ).collect::<String>()
    }
    fn as_vec ( &self ) -> Vec<String> {
        std::iter::once(String::from("."))
            .cycle()
            .take( self.size as usize )
            .collect::<Vec<String>>()
    }
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize
}
impl File {
    fn render ( &self ) -> String {
        std::iter::once(self.id.to_string())
            .cycle()
            .take( self.size as usize )
            .collect::<String>()
    }
    fn as_vec ( &self ) -> Vec<String> {
        std::iter::once(self.id.to_string())
            .cycle()
            .take( self.size as usize )
            .collect::<Vec<String>>()
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .flat_map(|ch| ch.to_digit(10).and_then(|d| Some(d as usize)))
        .collect::<Vec<usize>>();

    let mut base: Vec<DiskPartition> = Vec::new();
    for (ind, digit) in data.into_iter().enumerate() {
        // File
        if ind % 2 == 0 {
            base.push(DiskPartition::File(
                File {
                    id: ind / 2,
                    size: digit
                }
            ));

            continue;
        }

        // Free space
        base.push(DiskPartition::FreeSpace(
            FreeSpace {
                size: digit
            }
        ));
    }

    println!("Base string");
    for dp in &base {
        match dp {
            DiskPartition::File(f) => {
                print!("{}", f.render());
            },
            DiskPartition::FreeSpace(fs) => {
                print!("{}", fs.render());
            }
        }
    }
    println!("");

    println!("Base: {base:#?}");

    'outer: for inv_i in 0..base.len() {
        let i = base.len() - inv_i - 1;

        if let DiskPartition::File(f) = base[i].clone() {
            for j in 0..i {
                if i == j { continue; }
    
                if let DiskPartition::FreeSpace(fs) = base[j].clone() {
                    //println!("Checking partition {fs:?} for {}", f.size);

                    if fs.size == f.size {
                        base.swap(i, j);
                        continue 'outer;
                    }

                    if fs.size > f.size {
                        let to_insert = base.remove(i);

                        if let DiskPartition::FreeSpace(fs) = &mut base[j] {
                            fs.size -= f.size;
                        }
                        base.insert(i, DiskPartition::FreeSpace(
                            FreeSpace {
                                size: f.size
                            }
                        ));
                        base.insert(j, to_insert);
                        continue 'outer;
                    }
    
                }
            }
        } else { continue; }

        /*
        println!("Base string");
        for dp in &base {
            match dp {
                DiskPartition::File(f) => {
                    print!("{}", f.render());
                },
                DiskPartition::FreeSpace(fs) => {
                    print!("{}", fs.render());
                }
            }
        }
        println!(""); */
    }

    println!("Final string");
    for dp in &base {
        match dp {
            DiskPartition::File(f) => {
                print!("{}", f.render());
            },
            DiskPartition::FreeSpace(fs) => {
                print!("{}", fs.render());
            }
        }
    }
    println!("");

    let mut base_vec = Vec::new();
    for dp in &base {
        match dp {
            DiskPartition::File(f) => {
                base_vec.append(f.as_vec().as_mut());
            },
            DiskPartition::FreeSpace(fs) => {
                base_vec.append(fs.as_vec().as_mut());
            }
        }
    }

    let mut checksum = 0u128;
    for i in 0..base_vec.len() {
        if base_vec[i] == "." {
            continue;
        }

        checksum += i as u128 * base_vec[i].parse::<u128>().unwrap();
    }
    println!("Checksum: {checksum}");
}
