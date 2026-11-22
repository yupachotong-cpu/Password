const readline = require('readline');

// สร้าง interface สำหรับรับค่าจากผู้ใช้ผ่าน Terminal
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// ฟังก์ชันช่วยถามคำถามแบบ Promise เพื่อให้อ่านง่ายและไม่ตัดช่องว่างหัวท้ายของรหัสผ่าน
function askQuestion(query) {
  return new Promise((resolve) => {
    rl.question(query, (answer) => {
      resolve(answer);
    });
  });
}

// ฟังก์ชันตรวจสอบรหัสผ่านตามเงื่อนไข OWASP
function validatePassword(password) {
  // ข้อ 4: ห้าม trim() ช่องว่างหัวท้าย และรองรับ Unicode
  // ใช้ [...password].length เพื่อนับจำนวนตัวอักษร Unicode (เช่น อีโมจิ หรือสระภาษาไทย) ได้อย่างถูกต้อง
  const charLength = [...password].length;

  // ข้อ 2: ยอมรับรหัสผ่านที่มีความยาวตั้งแต่ 15 ตัวอักษรขึ้นไป
  if (charLength < 15) {
    return {
      isValid: false,
      reason: `รหัสผ่านสั้นเกินไป (ปัจจุบันมี ${charLength} ตัวอักษร ต้องมีอย่างน้อย 15 ตัวอักษร)`
    };
  }

  // ข้อ 3: ไม่บังคับว่าต้องมีตัวพิมพ์ใหญ่ ตัวพิมพ์เล็ก ตัวเลข หรือสัญลักษณ์พิเศษ
  // ข้อ 4: อนุญาตให้ใช้ช่องว่างและอักขระ Unicode ได้
  // ข้อ 5: รองรับความยาวอย่างน้อย 64 ตัวอักษร และห้ามตัดให้สั้นลงโดยไม่แจ้งผู้ใช้
  // (JavaScript String รองรับความยาวได้มหาศาลและไม่ตัดข้อความทิ้งอยู่แล้ว)

  return {
    isValid: true,
    reason: `รหัสผ่านมีความปลอดภัยตามเกณฑ์ (ความยาว ${charLength} ตัวอักษร)`
  };
}

async function main() {
  try {
    // ข้อ 1: รับ username (email) และ password จากผู้ใช้
    const rawUsername = await askQuestion('กรุณากรอก Username (Email): ');
    const username = rawUsername.trim(); // username สามารถตัดช่องว่างหัวท้ายได้ปกติ

    // รับ password โดยตรง ไม่ใช้ .trim() เพื่อรักษาช่องว่างหัวท้ายตามข้อ 4
    const password = await askQuestion('กรุณากรอก Password: ');

    // ตรวจสอบเบื้องต้นว่ากรอก username หรือไม่
    if (!username) {
      console.log('\n[ผลการตรวจสอบ] ไม่ผ่าน: กรุณากรอก Username (Email)');
      return;
    }

    // ตรวจสอบความถูกต้องของรหัสผ่าน
    const result = validatePassword(password);

    // ข้อ 6: แสดงผลว่าผ่านหรือไม่ผ่าน พร้อมเหตุผล โดยไม่พิมพ์ค่ารหัสผ่านออกมาซ้ำ และไม่บันทึกลงไฟล์
    console.log('\n--- ผลการตรวจสอบ ---');
    console.log(`บัญชีผู้ใช้: ${username}`);
    if (result.isValid) {
      console.log(`สถานะ: ผ่าน (PASS)`);
      console.log(`เหตุผล: ${result.reason}`);
    } else {
      console.log(`สถานะ: ไม่ผ่าน (FAIL)`);
      console.log(`เหตุผล: ${result.reason}`);
    }
  } finally {
    // ปิดการทำงานของ readline
    rl.close();
  }
}

main();