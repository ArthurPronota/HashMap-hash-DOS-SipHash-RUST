use std::collections::HashMap ; // Хэш-карта, реализованная с помощью квадратичного зондирования и поиска SIMD.
use ahash::RandomState ;    // хеш-конструктор (hasher builder) (Hasher factory)

fn main() {
    let mut m: HashMap<
                u64,
                u64,
                RandomState
               > = HashMap::default() ;

    for i in 0..20 {
        m.insert(i, i * 2) ;
    }          

    println!("{:#?}", m);
} /* Out:
{
    10: 20,
    9: 18,
    12: 24,
    11: 22,
    19: 38,
    16: 32,
    7: 14,
    14: 28,
    3: 6,
    13: 26,
    2: 4,
    1: 2,
    4: 8,
    17: 34,
    15: 30,
    5: 10,
    6: 12,
    8: 16,
    18: 36,
    0: 0,
}
*/

