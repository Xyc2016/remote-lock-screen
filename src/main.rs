use serde::Deserialize;

fn lock_screen() -> windows::core::Result<()>{
    unsafe {
        windows::Win32::System::Shutdown::LockWorkStation()
    }
}
const BASE_URL : &str = "http://xyc986327386.online:8008";
#[derive(Debug, Deserialize)]
struct GetLock {
    lock: bool,
}
#[tokio::main]
async fn main() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        match reqwest::get(format!("{BASE_URL}/get_lock")).await {
            Ok(res) => {
                dbg!(&res);
                match res.json::<GetLock>().await {
                    Ok(inner) => {
                        dbg!(&inner);
                        if inner.lock {
                            match lock_screen() {
                                Ok(_) => {},
                                Err(e) => {
                                    eprintln!("Failed to lock screen {}", e);
                                }
                            }
                        }
                    },
                    Err(_) => {
                        continue;
                    }
                }
            },
            Err(_) => {
                continue;
            }
        };
    }

}