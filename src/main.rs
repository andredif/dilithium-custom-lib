use dilithium_custom_lib::*;



fn main() {
    let seed = [95, 33, 185, 136, 150, 102, 233, 78, 10, 35, 186, 202, 124, 238, 99, 252, 168, 109, 133, 52, 199, 55, 127, 221, 134, 85, 109, 153, 159, 72, 74, 6];
    let keys1 = Keypair::generate(&seed);
    let keys2 = Keypair::generate(&seed);
    println!("Public key is: {:?}", keys1.public);
    println!("Private key is: {:?}", keys1.expose_secret());
    assert_eq!(keys1.public, keys2.public);
    assert_eq!(keys1.expose_secret(), keys2.expose_secret());
}
