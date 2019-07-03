use lightosd::Config;

fn main() {
    let opt = Config::load();
    lightosd::hello(&opt);
}
