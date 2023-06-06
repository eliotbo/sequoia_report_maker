import svgwrite
import numpy as np
import cairosvg
from math import cos, sin, pi, radians
import legend as lg
import config

def make_audiogram(dwg, left_ear: bool):
    # Audiogram data for one ear
    audiogram_data = [
        {'frequency': 125, 'dB': 20},
        {'frequency': 250, 'dB': 15},
        {'frequency': 500, 'dB': 10},
        {'frequency': 750, 'dB': 20},
        {'frequency': 1000, 'dB': 30},
        {'frequency': 1500, 'dB': 40},
        {'frequency': 2000, 'dB': 35},
        {'frequency': 3000, 'dB': 30},
        {'frequency': 4000, 'dB': 25},
        {'frequency': 6000, 'dB': 25},
        {'frequency': 8000, 'dB': 30},
    ]

    all_freqs = [
        125,
        187.5,
        250,
        375,
        500,
        750,
        1000,
        1500,
        2000,
        3000,
        4000,
        6000,
        8000,
    ]


    grid_lines = [
        125,
        250,
        500,
        1000,
        2000,
        4000,
        8000,
    ]

    grid_dash_lines = [750, 1500, 3000, 6000]
    ca_lines = [250, 500, 750, 1000, 1500, 2000, 3000, 4000, 6000, 8000]
    co_lines = [250, 500, 750, 1000, 1500, 2000, 3000, 4000]



    graph_width = 520
    graph_height = 520

    # We want our frequencies to appear at these relative positions along the x axis
    x_positions = np.linspace(0, graph_width, len(all_freqs))
    box_width =  x_positions[1] - x_positions[0] 
    box_height = graph_height / 13


    # A helper function to get x position for a frequency
    def freq_to_x(frequency):
        # index = [data['frequency'] for data in audiogram_data].index(frequency)
        # print( [data['frequency'] for data in audiogram_data])
        # print()
        shift_factor = graph_width + 300 if left_ear else 0
        index = all_freqs.index(frequency)
        return x_positions[index] + 100 + shift_factor

    # Now dB to y places higher dB values lower, with new y range from -10 to 120
    def dB_to_y(dB):
        return (dB) * box_height / 10 + 100

    
    # Create SVG



    # gray_num = 70
    # gray = f"rgb({gray_num}, {gray_num}, {gray_num})"
    gray = config.gray

    # # Add the following line to enable anti-aliasing
    # dwg['shape-rendering'] = 'geometricPrecision'

    start_x = freq_to_x(125)
    end_x = freq_to_x(8000)
    start_y = dB_to_y(-10)
    end_y = dB_to_y(120)


    ca_space = 20
    ca_y_start = end_y + ca_space
    ca_y_end = ca_y_start + 2 * box_height
    tick_offset = 8

    y_axis_pos = start_x - tick_offset
    y_axis_text_anchor = "end"
    y_axis_label_x_pos = start_x -tick_offset - 30
    y_axis_label_y_pos = dB_to_y(50) + 25
    y_axis_label_rot = f"rotate(-90 {y_axis_label_x_pos} {y_axis_label_y_pos})"
    ca_label_x_pos = y_axis_pos
    ca_anchor = "end"


    if left_ear:
        y_axis_pos = end_x + tick_offset
        y_axis_text_anchor = "start"
        y_axis_label_y_pos = dB_to_y(50) - 25
        y_axis_label_x_pos =   end_x + tick_offset + 30
        y_axis_label_rot = f"rotate(90 {y_axis_label_x_pos} {y_axis_label_y_pos})"
        ca_label_x_pos = end_x + box_width / 2 + tick_offset
        ca_anchor = "start"

    # Draw grid and labels from -10 to 120
    for i in range(-10, 121, 10):
        y = dB_to_y(i)
        dwg.add(dwg.line(start=(start_x, y), end=(end_x, y), stroke='lightgray')) 
        # if i == 0:
        #     dx = -30
        #     dwg.add(dwg.text(f"0", insert=(start_x - tick_offset + dx, y+6), text_anchor="end", font_size="20px", font_family="Fira Code")) 
        #     dwg.add(dwg.text(f"dBHL", insert=(start_x - tick_offset + dx + 2, y+6), text_anchor="start", font_size="15px", font_family="Fira Code")) 
        # else:
        if i==-10:
            continue
        dwg.add(dwg.text(f"{i}", insert=(y_axis_pos, y+6), text_anchor=y_axis_text_anchor, font_size="20px", font_family="Lato, Fira Code, sans-serif", fill=gray)) 

    for gl in grid_lines:
        x = freq_to_x(gl)
        dwg.add(dwg.line(start=(x, start_y), end=(x, end_y), stroke='lightgray')) 
        if gl == 125:
            dwg.add(dwg.text("125 Hz", insert=(x, start_y - tick_offset),  text_anchor="middle", font_size="20px",  font_family="Lato, Fira Code, sans-serif", fill=gray))
            continue

        dwg.add(dwg.text(f"{gl}", insert=(x, start_y - tick_offset),  text_anchor="middle", font_size="20px",  font_family="Lato, Fira Code, sans-serif", fill=gray)) # transform=f"rotate(-45 {x-20} 560)",


    # Draw axes
    # dwg.add(dwg.line(start=(start_x, start_y), end=(start_x, end_y), stroke='black')) 
    dwg.add(dwg.line(start=(start_x,dB_to_y(20)), end=(end_x,dB_to_y(20)), stroke='black', stroke_width=2)) 

    # Add units per axis
    unit_delta = 20
    # dwg.add(dwg.text("Hz", insert=(freq_to_x(1000), end_y + unit_delta),  text_anchor="middle", font_size="20px", font_family="Fira Code")) 
    dwg.add(dwg.text("dB HL", insert=(y_axis_label_x_pos, y_axis_label_y_pos), font_size="20px", transform=y_axis_label_rot, font_family="Lato, Fira Code, sans-serif", fill=gray)) 

    # text = dwg.text(
    #     'A', 
    #     insert=("300px", "315px"), 
    #     font_size="30px", 
    #     text_anchor="middle", 
    #     # dominant_baseline="middle", 
    #     fill="black",
    #     transform='rotate(45 300 300)'
    # )
    # dwg.add(text)

    # Frequencies for dashed lines
    dash_freqs = [750, 1500, 3000, 6000]

    for freq in dash_freqs:
        x = freq_to_x(freq)
        dwg.add(dwg.line(start=(x, start_y), end=(x, end_y), stroke='lightgray', stroke_dasharray="5,7"))


    # ca_x_start
    # 
    # Draw lines between points before drawing the points to ensure the points appear on top
    lines = []
    points = []
    last_point = None
    side_length = 10
    for data in audiogram_data:
        point = (freq_to_x(data['frequency']), dB_to_y(data['dB']))
        if last_point is not None:
            lines.append(dwg.line(start=last_point, end=point, stroke='black', stroke_width=2, stroke_dasharray="15,15"))

        # points.append(dwg.rect(insert=(point[0]-side_length/2, point[1]-side_length/2), size=(side_length,side_length), fill='white', stroke='red')) 
        last_point = point
        points.append(point)


    for line in lines:
        dwg.add(line)

    for point in points:
        pos = {"x": point[0], "y": point[1]}
        dwg = lg.add_square(
            dwg,
            10,
            pos,
            'red'
        )


    # ca co
    for freq in ca_lines:
        x = freq_to_x(freq)
        dwg.add(dwg.line(start=(x - box_width/2, ca_y_start), end=(x - box_width/2, ca_y_end), stroke='lightgray'))
        dwg.add(dwg.line(start=(x + box_width/2, ca_y_start), end=(x + box_width/2, ca_y_end), stroke='lightgray'))

    dwg.add(dwg.line(start=(start_x, ca_y_start), end=(start_x, ca_y_end), stroke='lightgray'))
    dwg.add(dwg.line(start=(start_x, ca_y_start), end=(end_x + box_width/2, ca_y_start), stroke='lightgray'))
    dwg.add(dwg.line(start=(start_x, ca_y_start+box_height), end=(end_x + box_width/2, ca_y_start + box_height), stroke='lightgray'))
    dwg.add(dwg.line(start=(start_x, ca_y_end), end=(end_x + box_width/2, ca_y_end), stroke='lightgray'))


    dwg.add(dwg.rect(insert=(start_x, ca_y_start), size=(box_width* 1.5,box_height * 2), fill='lightgray', stroke='none'))
    dwg.add(dwg.rect(insert=(start_x + box_width *2.5, ca_y_start), size=(box_width,box_height * 2), fill='lightgray', stroke='none'))

    # x = freq_to_x(freq)
    dwg.add(dwg.rect(insert=(end_x - box_width *1.5, ca_y_start + box_height), size=(box_width * 2,box_height), fill='lightgray', stroke='none'))

    dwg.add(dwg.text("CA", insert=(ca_label_x_pos, ca_y_start + box_height - 13), font_size="20px",   text_anchor=ca_anchor, font_family="Lato, Fira Code, sans-serif", fill=gray)) 
    dwg.add(dwg.text("CO", insert=(ca_label_x_pos, ca_y_start + box_height * 2 - 13 ), font_size="20px",  text_anchor=ca_anchor, font_family="Lato, Fira Code, sans-serif", fill=gray)) 


    return dwg



import svgwrite
import webbrowser
import os


inverse_scale = 0.6
dwg = svgwrite.Drawing('audiogram.svg', profile='tiny', 
    # size=("1020px", "876px"))
    size=(2550 * inverse_scale, 3300 * inverse_scale))

left_ear = False

dwg = make_audiogram(dwg, left_ear)
right_ear = True
dwg = make_audiogram(dwg, right_ear)




def add_red_disk(dwg, pos, r=2):
    dwg.add(dwg.circle(center=(pos["x"], pos["y"]), r=r, fill='red'))
    return dwg



positions = [{"x": 30, "y": y} for y in range(40, 640, 40)]
functions = [lg.add_s, lg.add_a, lg.add_asterisk, lg.add_vt, lg.add_u,
             lg.add_less_than, lg.add_greater_than, lg.add_x, lg.add_right_bracket, 
             lg.add_left_bracket, lg.add_bottom_left_arrow, lg.add_bottom_right_arrow,
             lg.add_triangle, lg.add_circle, lg.add_square]

for pos, func in zip(positions, functions):
    dwg = func(dwg, 13, pos)
    dwg = add_red_disk(dwg, pos)

legend = lg.Legend()
# unused_symbols = ["Inconfort", "Surassourdissement ou\nmasque insuffisant", "Masqué "]
unused_symbols = []
dwg = legend.draw_legend(dwg, {"width": 250, "height": 800}, {"top": 60, "left": 645}, unused_symbols)


# Save SVG
dwg.save()


# Convert to PDF
cairosvg.svg2pdf(
    file_obj=open("audiogram.svg", "rb"),  
    write_to="output5.pdf", 
    parent_width=1050,
    parent_height=876,
    output_width=2550,
    output_height=3300,
    # scale=10.0,
    dpi=300
)


# # Open SVG in browser
filename = 'file://' + os.path.realpath(dwg.filename)
webbrowser.open_new_tab(filename)

import pyautogui

# Open your browser using Python, then run your script

# Then use pyautogui to switch focus
pyautogui.hotkey('alt', 'tab')
