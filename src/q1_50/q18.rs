pub fn q18() -> i64 {

    let triangle_str: String = "75
                                95 64
                                17 47 82
                                18 35 87 10
                                20 04 82 47 65
                                19 01 23 75 03 34
                                88 02 77 73 07 63 67
                                99 65 04 28 06 16 70 92
                                41 41 26 56 83 40 80 70 33
                                41 48 72 33 47 32 37 16 94 29
                                53 71 44 65 25 43 91 52 97 51 14
                                70 11 33 28 77 73 17 78 39 68 17 57
                                91 71 52 38 17 14 91 43 58 50 27 29 48
                                63 66 04 68 89 53 67 30 73 16 69 87 40 31
                                04 62 98 27 23 09 70 98 73 93 38 53 60 04 23 "
        .to_string();


    let mut tri_vec: Vec<&str> = triangle_str.split_whitespace().collect();
    tri_vec.insert(0, ""); // have heap data now

    for s in &tri_vec {
        println!("{}", s);
    }

    println!("{}", tri_vec[0]);


    return -1;
}


fn max_depth(heap: &Vec<&str>, index: u8, size: u8) -> i64 {


    if 2 * index <= size {
        max_depth(&heap, 2 * index, size); //left child
        max_depth(&heap, 2 * index + 1, size); //right child
    }




}
