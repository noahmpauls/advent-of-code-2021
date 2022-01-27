use std::sync::{mpsc, Arc, RwLock};
use std::thread;

pub fn monad_max_threaded(program: &Program, thread_count: usize) -> Option<String> {
    let memo: Arc<RwLock<Memo>> = Arc::new(RwLock::new(HashSet::new()));
    let (tx, rx) = mpsc::channel();

    (0..thread_count).into_iter().for_each(|t| {
        let program = program.clone();
        let memo_clone = memo.clone();
        let tx_clone = tx.clone();
        let r = 9 / thread_count;
        let start = r * t;
        let mut end = r * (t + 1);
        if 9 - end <= 9 % r {
            end = 9;
        }

        println!("start, end: {}, {}", start, end);

        thread::spawn(move || {
            let result = monad_max_range(&program, start as Reg, end as Reg, memo_clone);
            tx_clone.send(result).unwrap();
        });
    });

    let results: Vec<Option<String>> = (0..thread_count).into_iter().map(|_| {
        let received = rx.recv().unwrap();
        println!("received a response! {:?}", received);
        received
    }).collect();

    let max = results.into_iter().filter_map(|x| x.map(|x| x.parse::<i128>().unwrap())).max();

    max.map(|m| m.to_string())
}

fn monad_max_range(program: &Program, start: Reg, end: Reg, memo: Arc<RwLock<Memo>>) -> Option<String> {
    let blocks = program.into_blocks();
    let state = Registers { w:0, x:0, y:0, z:0 };
    let mut local_memo = Memo::new();
    let mut hit_count = 0;
    monad_max_block_range(&blocks, 0, state, start, end, &mut local_memo, &mut hit_count, memo).map(|result| {
        result.into_iter().rev()
            .map(|digit| digit.to_string())
            .collect()
    })
}

fn monad_max_block_range(blocks: &[Program], block: usize, state: Registers, start: Reg, end: Reg, local_memo: &mut Memo, hit_count: &mut u128, memo: Arc<RwLock<Memo>>) -> Option<Vec<Reg>> {
    const HIT_COUNT: u128 = 1;
    
    for input in (start+1..end+1).rev() {
        if let Ok(memo_read) = memo.read() {
            if memo_read.contains(&MemoEntry{ block, input, z: state.z }) {
                continue;
            }
        }


        let mut alu = ALU::new(&[input]);
        alu.set_state(&state);
        alu.run(&blocks[block]);
        let output = alu.state();
        
        // blocks left to analyze
        if block + 1 < blocks.len() {
            if let Some(mut digits) = monad_max_block_range(&blocks, block + 1, output, 0, 9, local_memo, hit_count, memo.clone()) {
                digits.push(input);
                return Some(digits);
            } else {
                local_memo.insert(MemoEntry{ block, input, z: state.z });
                *hit_count += 1;
                if *hit_count % HIT_COUNT == 0 {
                    if let Ok(mut memo_write) = memo.write() {
                        for entry in local_memo.drain() {
                            memo_write.insert(entry);
                        }
                    }
                }
            }
        // end reached
        } else {
            if output.z == 0 {
                return Some(vec![input]);
            } else {
                local_memo.insert(MemoEntry{ block, input, z: state.z });
                *hit_count += 1;
                if *hit_count % HIT_COUNT == 0 {
                    if let Ok(mut memo_write) = memo.write() {
                        for entry in local_memo.drain() {
                            memo_write.insert(entry);
                        }
                    }
                }
            }
        }
    }

    None
}