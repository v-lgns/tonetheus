use tonetheus::ton::MyTonCtrl;

fn main() {
    let mut mytonctrl = MyTonCtrl::new();
    let status = mytonctrl.status();
    dbg!(status);
}
