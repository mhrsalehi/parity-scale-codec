
#[test]
fn generate_u8_tests_for_polkadot4j() {
    use hex::encode;
    type T = u16;
    let mut x : T = 1;
    let mut test_vectors: Vec<(T, String)> = Vec::new();
    let mut done = false;
    while x <= T::MAX {
        test_vectors.push((x - 1, encode((x - 1).encode())));
        test_vectors.push((x, encode((x).encode())));
        if x < T::MAX {
            test_vectors.push((x + 1, encode((x + 1).encode())));
        }
        if done {
            break;
        }
        x = x << 1;
        if x == 0 {
            x = T::MAX;
            done = true;
        }
    }
    for i in test_vectors.iter() {
        // println!("t(new BigInteger(\"{:?}\"), {:?}),", i.0, i.1);
        println!("t({:?}, {:?}),", i.0, i.1);
    }
}


#[test]
fn generate_u8_tests_for_polkadot4j_uuuuu() {
    use hex::encode;
    type T = u16;
    type T2 = i16;
    let mut x : T = 1;
    let mut test_vectors: Vec<(T2, String)> = Vec::new();
    let mut done = false;
    while x <= T::MAX {
        test_vectors.push(((x - 1) as T2, encode(((x - 1) as T2).encode())));
        test_vectors.push((x as T2, encode((x as T2).encode())));
        if x < T::MAX {
            test_vectors.push(((x + 1) as T2, encode(((x + 1) as T2).encode())));
        }
        if done {
            break;
        }
        x = x << 1;
        if x == 0 {
            x = T::MAX;
            done = true;
        }
    }
    x = 1;
    done = false;
    while x <= T::MAX {
        let y = x;
        x = x | (1 << 15);
        test_vectors.push(((x - 1) as T2, encode(((x - 1) as T2).encode())));
        test_vectors.push((x as T2, encode((x as T2).encode())));
        if x < T::MAX {
            test_vectors.push(((x + 1) as T2, encode(((x + 1) as T2).encode())));
        }
        if done {
            break;
        }
        x = x << 1;
        if x == 0 {
            x = T::MAX;
            done = true;
        }
    }
    test_vectors.sort_by(|a, b| a.1.cmp(&b.1));
    test_vectors.dedup();
    for i in test_vectors.iter() {
        // println!("t(new BigInteger(\"{:?}\"), {:?}),", i.0, i.1);
        println!("t({:?}, {:?}),", i.0, i.1);
    }
}