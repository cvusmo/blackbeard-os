import OPi.GPIO as GPIO
import time

# Set the mode of numbering the pins. 
GPIO.setmode(GPIO.BOARD)

# Set up a channel (pin number 7 in this example) 
GPIO.setup(7, GPIO.OUT)

# Blink the LED
try:
    while True:
        GPIO.output(7, GPIO.HIGH)  # LED on
        time.sleep(1)               # Delay for 1 second
        GPIO.output(7, GPIO.LOW)   # LED off
        time.sleep(1)               # Delay for 1 second
except KeyboardInterrupt:
    GPIO.cleanup()                 # Clean up the port before exiting

