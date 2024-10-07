fn main() {
    unsafe {
        windows::Win32::System::Shutdown::LockWorkStation()
    }.expect("Failed to lock workstation");
}
