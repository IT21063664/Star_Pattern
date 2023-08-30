fn main() {
    let mut i = 0;
    let mut n = 0;
    let mut j;
    let mut c = 0;

    while i < 5 {
        j = 5 - i;
        //print!("{}", j);
        //print!("{}", i);

        while j > 0 {
            print!(" ");
            j = j - 1;
        }

        n = i + 1;
        while c < i + n {
            print!("*");
            n = n - 1;

            if c == n + i {
                break;
            }
        }

        println!();
        i = i + 1;
    }
}
