#define SHIFT_DATA 2
#define SHIFT_CLOCK 3
#define SHIFT_LATCH_OUTPUT 4
#define EEPROM_ENABLE 5
#define EEPROM_LSB 6
#define EEPROM_MSB 13

const bool BIG_CHIP = false;

const int ADDRESS_MASK_BIG =   (0b00011111 << 8) + 0xff;
const int WRITE_ENABLE_BIG =    0b00100000 << 8;
const int ADDRESS_MASK_SMALL = (0b00000111 << 8) + 0xff;
const int WRITE_ENABLE_SMALL =  0b00001000 << 8;
const int OUTPUT_ENABLE =       0b10000000 << 8;

const int ADDRESS_MASK = BIG_CHIP ? ADDRESS_MASK_BIG : ADDRESS_MASK_SMALL;
const int WRITE_ENABLE = BIG_CHIP ? WRITE_ENABLE_BIG : WRITE_ENABLE_SMALL;

void setAddress(int address, bool writing) {
  int masked = address & ADDRESS_MASK;
  int control_bits = writing ? OUTPUT_ENABLE : WRITE_ENABLE; // backwards bc active low
  masked = masked | control_bits;
  
  shiftOut(SHIFT_DATA, SHIFT_CLOCK, MSBFIRST, masked >> 8);
  shiftOut(SHIFT_DATA, SHIFT_CLOCK, MSBFIRST, masked);
  digitalWrite(SHIFT_LATCH_OUTPUT, HIGH);
  digitalWrite(SHIFT_LATCH_OUTPUT, LOW);
}
void setAddressWrite(int address) {
    setAddress(address, true);
}
void setAddressRead(int address) {
    setAddress(address, false);
}

void writeEEPROM(int address, byte data) {
  digitalWrite(EEPROM_ENABLE, HIGH);
  setAddressWrite(address);
  for (int pin = EEPROM_LSB; pin <= EEPROM_MSB; pin += 1) {
    pinMode(pin, OUTPUT);
    digitalWrite(pin, data & 1);
    data = data >> 1;
  }
  digitalWrite(EEPROM_ENABLE, LOW);
  delayMicroseconds(1);
  digitalWrite(EEPROM_ENABLE, HIGH);
  delay(10);
}

byte readEEPROM(int address) {
  setAddressRead(address);
  digitalWrite(EEPROM_ENABLE, LOW);
  byte data = 0;
  for (int pin = EEPROM_MSB; pin >= EEPROM_LSB; pin -= 1) {
    pinMode(pin, INPUT);
    data = (data << 1) + digitalRead(pin);
  }
  return data;
}

void dumpEEPROM() {
  for (int base = 0; base <= ADDRESS_MASK; base += 16) {
    byte data[16];
    for (int offset = 0; offset <= 15; offset += 1) {
      data[offset] = readEEPROM(base + offset);
    }

    char buf[80];
    sprintf(buf, "%03x:  %02x %02x %02x %02x %02x %02x %02x %02x   %02x %02x %02x %02x %02x %02x %02x %02x",
            base, data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]);

    Serial.println(buf);
  }
}

//=======================================================================

#define CO 0b00100000 << 8
#define RO 0b01000000 << 8
#define TEST1O 0b01100000 << 8
#define TEST2O 0b10000000 << 8
#define EO 0b10100000 << 8
#define AO 0b11000000 << 8
#define BO 0b11100000 << 8

#define J  0b00001000 << 8
#define JZ 0b00010000 << 8
#define JC 0b00011000 << 8

#define II 0b00000100 << 8
#define MI 0b00000010 << 8
#define RI 0b00000001 << 8


#define AI 0b10000000
#define BI 0b01000000
#define OI 0b00100000
//#define  0b00010000
#define SR 0b00001000
#define CE 0b00000100
#define SU 0b00000010
#define HL 0b00000001

const bool HIGH_CHIP = false;

struct Instruction {
  uint16_t getStep(byte stp){ return this->steps[stp]; }
  uint16_t steps[8];
};

struct Instruction getInstruction(byte op) {
  switch(op) {
    case 0: // 0x00 TESTING
//      return Instruction{ 0, TEST2O|II, AO|J|MI, BO|JZ|RI, EO|II, TEST1O|II|MI|J, CO|JC|MI|RI, RO|RI|JZ };
      return Instruction{ 0, TEST2O, AO, BO, EO, TEST1O, CO, RO };

    case 0x10:
      return Instruction{ CO|II, CE, CO|BI, 0, 0, 0, 0, 0};
    case 0x11:
      return Instruction{ CO|II, CE, CO|AI, SU|EO|BI, 0, 0, 0, 0};
  
    
    default:
        return Instruction{ CO|II, CE, EO|AI, 0, 0, 0, 0, 0};
//      return Instruction{ CO|MI, RO|II|CE, 0, 0, 0, 0, 0, 0 };
  }
}

void writeInstructions() {
  if (BIG_CHIP) {
    Serial.println("BIG_CHIP must be false! Skipping");
    return;
  }
  if (HIGH_CHIP) {
    Serial.print("[ Writing HI.... ]\n[");
  } else {
    Serial.print("[ Writing LOW... ]\n[");
  }
  for (int op = 0; op < 256; op++) {
    Instruction instr = getInstruction(op);
    for (int stp = 0; stp < 8; stp++) {
      uint16_t step_instr = instr.getStep(stp);
      if (HIGH_CHIP) {
        step_instr = step_instr >> 8;
      }
      writeEEPROM(op + (stp << 8), 0xff ^ step_instr);
    }
    if (op % 16 == 15) {
      Serial.print(".");
    }
  }
  Serial.println("]");
}


//=======================================================================

void setupPins() {
  pinMode(SHIFT_DATA, OUTPUT);
  pinMode(SHIFT_CLOCK, OUTPUT);
  pinMode(SHIFT_LATCH_OUTPUT, OUTPUT);
  digitalWrite(SHIFT_LATCH_OUTPUT, LOW);
  pinMode(EEPROM_ENABLE, OUTPUT);
  digitalWrite(EEPROM_ENABLE, HIGH); //disable chip by default
}

void setup() {
  Serial.begin(115200);
  setupPins();


//  for (int i = 0; i <= 255; i += 1) {
//    writeEEPROM(i, (i+3)%32);
//  }
  writeInstructions();
  dumpEEPROM();
}

void loop() {
  // put your main code here, to run repeatedly:
}