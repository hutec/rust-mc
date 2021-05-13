mod euler;
mod pi;
mod birthday;

fn main() {
    pi::estimate_pi();
    euler::estimate_e();
    birthday::birthday_sharing();
}
