use std::io::{self, Write};

fn main() {
    // 1. รับ Username (Email)
    print!("Enter Username (Email): ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    // 1. รับ Password
    print!("Enter Password: ");
    io::stdout().flush().unwrap();
    let mut raw_password = String::new();
    io::stdin().read_line(&mut raw_password).unwrap();

    // 4. ตัดช่องว่างหัวท้าย (Trim)
    let password = raw_password.trim();
    let char_count = password.chars().count();
    let mut errors = Vec::new();

    // 2 & 5. ตรวจสอบความยาวอย่างน้อย 15 ตัวอักษร
    if char_count < 15 {
        errors.push(String::from("รหัสผ่านสั้นเกินไป: ต้องมีความยาวอย่างน้อย 15 ตัวอักษร"));
    }

    // 5. แจ้งเตือนถ้ายาวเกิน 64 ตัวอักษร (โดยไม่ตัดทอน)
    if char_count > 64 {
        println!("[แจ้งเตือน]: รหัสผ่านมีความยาว {} ตัวอักษร (รองรับและตรวจสอบตลอดความยาวโดยไม่ตัดทอน)", char_count);
    }

    // 6. แสดงผลลัพธ์ (ไม่ปริ้นท์รหัสผ่านซ้ำ)
    let is_valid = errors.is_empty();
    println!("\n--- ผลการตรวจสอบ ---");
    println!("User: {}", username);
    if is_valid {
        println!("สถานะ: ผ่าน (Password Valid)");
    } else {
        println!("สถานะ: ไม่ผ่าน (Password Invalid)");
        println!("เหตุผล:");
        for err in errors {
            println!("- {}", err);
        }
    }
}