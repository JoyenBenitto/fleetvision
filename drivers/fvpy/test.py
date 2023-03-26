from tkinter import *
import random

with open('log.csv', 'r') as f:
    [print(line.strip()) for line in f.readlines()]

def update_gauge(id_needle):
    newvalue = random.randint(low_r, hi_r)  
    cnvs.itemconfig(id_text,text = str(newvalue) + " %")
    # Rescale value to angle range (0%=120deg, 100%=30 deg)
    angle = 120 * (hi_r - newvalue)/(hi_r - low_r) + 30
    cnvs.itemconfig(id_needle,start = angle)
    root.after(3000, update_gauge(id_needle))
      
# Create Canvas objects    
 
canvas_width = 400
canvas_height =300
 
root = Tk()
 
cnvs = Canvas(root, width=canvas_width, height=canvas_height)
cnvs.grid(row=2, column=1)
 
coord_1 = 10, 50, 350, 350 
low_r = 0 
hi_r = 100

coord_2 = 210, 50, 350, 350
 
# Create a background arc and a pointer (very narrow arc)
cnvs.create_arc(coord_1, start=30, extent=120, fill="white",  width=2)
id_needle_1 = cnvs.create_arc(coord_1, start= 119, extent=1, width=7)

cnvs.create_arc(coord_2, start=30, extent=120, fill="white",  width=2)
id_needle_2 = cnvs.create_arc(coord_2, start= 119, extent=1, width=7)

# Add some labels
cnvs.create_text(180,20,font="Times 20 bold", text="Speed")
cnvs.create_text(25,140,font="Times 12 bold", text=low_r)
cnvs.create_text(330,140,font="Times 12 bold", text=hi_r)
id_text = cnvs.create_text(170,210,font="Times 15 bold")
 
root.after(3000, update_gauge(id_needle_1))
root.after(3000, update_gauge(id_needle_2))
 
root.mainloop()
