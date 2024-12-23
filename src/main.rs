// main.rs

use jwt_decoder::decode_jwt;  // Importer la fonction du fichier lib.rs

fn main() {
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.1-93Hefv2d6lJhyqVOv-2C5BtFo8VtF8okElb0y-MfU";


    let result = decode_jwt(jwt);
    println!("{}", result);
}
