fn main() {
    let mut a = vec![4, 2, 5, 1, 2, 7, 0, 5];
    bubble(&mut a);
    println!("{a:?}");
}

fn bubble(v: &mut Vec<isize>){
    let mut ok = true;
    let mut tr = 0;
    
    while ok{
        let mut ok_t = 0;
        for i in 0..v.len(){
            tr += 1;
            let one = v[i].clone();
            if i <= (v.len() - 2){
                let two = v[i + 1].clone();
                if one > two{
                    v[i] = two;
                    v[i + 1] = one;
                    ok_t += 1
                }
                ok = ok_t != 0;
            }
        }
    }
    println!("try: {tr}");
}
