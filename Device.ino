#include <Adafruit_MLX90614.h> 


int buttonstateA= 0;
int buttonstateB= 0;
int buttonstateC= 0;

int counter = 0;
int currentStateCLK;
int lastStateCLK;
String currentDir ="";
unsigned long lastButtonPress = 0;

char button ='A';


Adafruit_MLX90614 mlx = Adafruit_MLX90614();

void setup() {

  mlx.begin();
  
  pinMode(2, INPUT); // 2 ------> Button A
  pinMode(3, INPUT); // 3 ------> Button B
  pinMode(4, INPUT); // 4 ------> Button C
  pinMode(5, OUTPUT);// 5 ------> Vibration motor
  pinMode(6, INPUT); // 6 ------> IR
  pinMode(7, OUTPUT); // Vibration motor 1
  Serial.begin(9600); // opens serial port, sets data rate to 9600 bps
}


void loop() {
  //Serial.print("Ambient = "); Serial.print(mlx.readAmbientTempC());
 // Serial.print("*C\tObject = "); 
 Serial.print(mlx.readObjectTempC());
 Serial.print(",");

  buttonstateA = digitalRead(2);
  if (buttonstateA == HIGH) {
    button = 'A';
  }

  buttonstateB = digitalRead(3);
  if (buttonstateB == HIGH) {
    button = 'B';
  }

  buttonstateC = digitalRead(4);
  if (buttonstateC == HIGH) {
    button = 'C';
  }
  Serial.print(button);
  Serial.print(",");
 
  int IRStatus = digitalRead(6);
  if (IRStatus == 1) 
  {
    Serial.print("0"); 
  }
  else  {
    Serial.print("1"); 
  }
  int k = 0;
  if(k == 1){
    digitalWrite(7, HIGH);
    delay(3000);
    digitalWrite(7, LOW);
  }
  Serial.println();
  delay(1000);
  }

  
