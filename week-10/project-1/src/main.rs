struct Electronic {
  num: u32, price: u32
}

fn main () {

    let comp1 = Electronic {
        num: 10, price: 650_000
    };

    let comp2 = Electronic {
        num: 6, price: 755_000
    };

    let comp3 = Electronic {
        num: 10, price: 550_000
    };

    let comp4 = Electronic {
         num: 4, price: 850_000
    };

    let total_1 = comp1.num + comp2.num + comp3.num + comp4.num;
    let chk = comp1.price*3 + comp2.price*3 + comp3.price*3 + comp4.price*3;

    println!("There are {} HP laptops, {} IBM laptops, {} Toshiba laptops and {} Dell laptops.\n",
        comp1.num, comp2.num, comp3.num, comp4.num);
    println!("Together, there are {} laptops.\n Purchasing three of each brand will give you NGN {}."
                ,total_1,chk);
}