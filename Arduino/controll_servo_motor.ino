#include <Servo.h>

Servo myservo;  // create servo object to control a servo

void setup() {
    Serial.begin(9600)
    myservo.attach(9, 600, 2300);  // (pin, min, max)
}

void loop() {
    while (Serial.available() > 0) {
        int angle = Serial.parseInt();
        myservo.write(angle);
    }
    delay(1000);
}