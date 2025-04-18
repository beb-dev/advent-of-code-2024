#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Memory {
    File { id: usize, size: usize },
    FreeSpace { size: usize },
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day9.txt").trim();

    let mut diskmap1 = create_diskmap_part1(&input);
    let mut diskmap2 = create_diskmap_part2(&input);

    move_files_part1(&mut diskmap1);
    move_files_part2(&mut diskmap2);

    let part1 = get_checksum(&diskmap1);
    let part2 = get_checksum(&diskmap2);

    (part1.to_string(), part2.to_string())
}

fn create_diskmap_part1(input: &str) -> Vec<Memory> {
    let mut id = 0;
    let mut diskmap: Vec<Memory> = Vec::new();

    for (index, value) in input.chars().enumerate() {
        let size: usize = value.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            for _ in 0..size {
                diskmap.push(Memory::File { id, size: 1 });
            }
            id += 1;
        } else {
            for _ in 0..size {
                diskmap.push(Memory::FreeSpace { size: 1 });
            }
        }
    }

    diskmap
}

fn create_diskmap_part2(input: &str) -> Vec<Memory> {
    let mut id = 0;
    let mut diskmap: Vec<Memory> = Vec::new();

    for (index, value) in input.chars().enumerate() {
        let size: usize = value.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            diskmap.push(Memory::File { id, size });
            id += 1;
        } else {
            diskmap.push(Memory::FreeSpace { size });
        }
    }

    diskmap
}

fn move_files_part1(diskmap: &mut Vec<Memory>) {
    let mut temp_diskmap: Vec<Memory> = Vec::with_capacity(diskmap.len());

    let mut front = 0;
    let mut back = diskmap.len() - 1;

    while front <= back {
        let memory = diskmap[front];
        front += 1;

        match memory {
            Memory::File { id: _, size: _ } => {
                temp_diskmap.push(memory);
            }

            Memory::FreeSpace { size: _ } => {
                while back > front {
                    let memory = diskmap[back];
                    back -= 1;

                    if let Memory::File { id: _, size: _ } = memory {
                        temp_diskmap.push(memory);
                        break;
                    }
                }
            }
        }
    }

    temp_diskmap.clone_into(diskmap);
}

fn move_files_part2(diskmap: &mut Vec<Memory>) {
    let mut back = diskmap.len() - 1;

    while back > 0 {
        let memory = diskmap[back];

        if let Memory::File { id: _, size } = memory {
            let mut front = 0;

            while front < back {
                if let Memory::FreeSpace { size: free_space } = diskmap[front] {
                    if free_space == size {
                        diskmap.swap(back, front);
                        break;
                    } else if free_space > size {
                        let space_left = free_space - size;
                        diskmap[front] = Memory::FreeSpace { size };
                        diskmap.swap(back, front);
                        diskmap.insert(front + 1, Memory::FreeSpace { size: space_left });
                        break;
                    }
                }

                front += 1;
            }
        }

        back -= 1;
    }
}

fn get_checksum(diskmap: &Vec<Memory>) -> usize {
    let mut checksum = 0;
    let mut position = 0;

    for block in diskmap {
        match *block {
            Memory::File { id, size } => {
                for _ in 0..size {
                    checksum += position * id;
                    position += 1;
                }
            }
            Memory::FreeSpace { size } => {
                position += size;
            }
        }
    }

    checksum
}
