use std::io::{self, Write};

// โครงสร้างสำหรับเก็บผลการตรวจสอบ
struct ValidationResult {
    is_valid: bool,
    reason: String,
}

// ฟังก์ชันตรวจสอบรหัสผ่านตามเงื่อนไข OWASP
fn validate_password(password: &str) -> ValidationResult {
    // ข้อ 4: ห้าม trim() ช่องว่างหัวท้าย และรองรับ Unicode
    // ใช้ .chars().count() เพื่อนับจำนวนตัวอักษร Unicode จริง (ห้ามใช้ .len() เพราะจะนับเป็น Byte)
    let char_length = password.chars().count();

    // ข้อ 2: ยอมรับรหัสผ่านที่มีความยาวตั้งแต่ 15 ตัวอักษรขึ้นไป
    if char_length < 15 {
        return ValidationResult {
            is_valid: false,
            reason: format!(
                "รหัสผ่านสั้นเกินไป (ปัจจุบันมี {} ตัวอักษร ต้องมีอย่างน้อย 15 ตัวอักษร)",
                char_length
            ),
        };
    }

    // ข้อ 3: ไม่บังคับตัวพิมพ์ใหญ่ ตัวพิมพ์เล็ก ตัวเลข หรือสัญลักษณ์พิเศษ
    // ข้อ 4: อนุญาตให้ใช้ช่องว่างและอักขระ Unicode ได้
    // ข้อ 5: รองรับความยาวอย่างน้อย 64 ตัวอักษร และห้ามตัดให้สั้นลงโดยไม่แจ้งผู้ใช้
    // (ชนิดข้อมูล String ของ Rust ขยายขนาดอัตโนมัติตามหน่วยความจำ ไม่มีการตัดข้อความทิ้ง)

    ValidationResult {
        is_valid: true,
        reason: format!(
            "รหัสผ่านมีความปลอดภัยตามเกณฑ์ (ความยาว {} ตัวอักษร)",
            char_length
        ),
    }
}

fn main() {
    // ข้อ 1: รับ username (email) จากผู้ใช้
    print!("กรุณากรอก Username (Email): ");
    io::stdout().flush().expect("ไม่สามารถแสดงผลข้อความได้");

    let mut raw_username = String::new();
    io::stdin()
        .read_line(&mut raw_username)
        .expect("อ่านค่า Username ไม่สำเร็จ");
    let username = raw_username.trim(); // username ตัดช่องว่างหัวท้ายได้ปกติ

    // ข้อ 1: รับ password จากผู้ใช้
    print!("กรุณากรอก Password: ");
    io::stdout().flush().expect("ไม่สามารถแสดงผลข้อความได้");

    let mut raw_password = String::new();
    io::stdin()
        .read_line(&mut raw_password)
        .expect("อ่านค่า Password ไม่สำเร็จ");

    // ข้อ 4: ห้ามใช้ .trim() กับ password เพราะจะลบช่องว่าง (Space) หัวท้ายทิ้ง
    // ให้ตัดเฉพาะอักขระขึ้นบรรทัดใหม่ (\n และ \r) ที่เกิดจากการกดปุ่ม Enter เท่านั้น
    let password = raw_password.trim_end_matches(&['\r', '\n'][..]);

    // ตรวจสอบเบื้องต้นว่ากรอก username หรือไม่
    if username.is_empty() {
        println!("\n[ผลการตรวจสอบ] ไม่ผ่าน: กรุณากรอก Username (Email)");
        return;
    }

    // ตรวจสอบความถูกต้องของรหัสผ่าน
    let result = validate_password(password);

    // ข้อ 6: แสดงผลว่าผ่านหรือไม่ผ่าน พร้อมเหตุผล โดยไม่พิมพ์ค่ารหัสผ่านออกมาซ้ำ และไม่บันทึกลงไฟล์
    println!("\n--- ผลการตรวจสอบ ---");
    println!("บัญชีผู้ใช้: {}", username);
    if result.is_valid {
        println!("สถานะ: ผ่าน (PASS)");
        println!("เหตุผล: {}", result.reason);
    } else {
        println!("สถานะ: ไม่ผ่าน (FAIL)");
        println!("เหตุผล: {}", result.reason);
    }
}