import streamlit as st
import time
import schedule
import pandas as pd
from random import randint

placeholder1 =st.empty()
placeholder2 = st.empty()
placeholder3 = st.empty()
col1, col2, col3 = st.columns(3)


def speed():
    with col1:
        speed = randint(0, 90)
        placeholder2.metric(value=speed, label="speed")


def oil():
    with col2:
        oil = randint(0, 90)
        #spd_data = [{'Oil':oil}]
        #spd_df = pd.DataFrame(spd_data)
        placeholder3.metric(value=oil, label="oil")
 

def sensor3():
    with col3:
        sen = randint(0, 10)
        placeholder1.metric(value=sen, label="sensor 3")


def ir1():
    ir_1 = 0
    if ir_1 == True:
        st.metric(value=0, label = "ir_1 cone")


def ir2():
    ir_2 = 1
    if ir_2 == True:
        st.metric(value=1, label = "ir_1 cone")


def ir3():
    ir_3 = 1
    if ir_3 == True:
        st.metric(value=1, label = "ir_1 cone")


def ir4():
    ir_4 = 0
    if ir_4 == True:
        st.metric(value=0, label = "ir_1 cone")
    

schedule.every(1).seconds.do(speed)
schedule.every(1).seconds.do(oil)
schedule.every(1).seconds.do(sensor3)
ir1()
ir2()
ir3()
ir4()

while True:
    schedule.run_pending()
    time.sleep(2)


