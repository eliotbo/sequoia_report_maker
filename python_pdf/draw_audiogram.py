import svgwrite
import numpy as np
import cairosvg
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

# We want our frequencies to appear at these relative positions along the x axis
x_positions = np.linspace(0, 520, len(all_freqs))

# A helper function to get x position for a frequency
def freq_to_x(frequency):
    # index = [data['frequency'] for data in audiogram_data].index(frequency)
    # print( [data['frequency'] for data in audiogram_data])
    # print()
    index = all_freqs.index(frequency)
    return x_positions[index] + 100

# Now dB to y places higher dB values lower, with new y range from -10 to 120
def dB_to_y(dB):
    return (dB) * 4 + 100

ya = 0.6
# Create SVG
dwg = svgwrite.Drawing('audiogram.svg', profile='tiny', 
    # size=("1020px", "876px"))
    size=(2550 * ya, 3300 * ya))


gray_num = 70
gray = f"rgb({gray_num}, {gray_num}, {gray_num})"

# # Add the following line to enable anti-aliasing
# dwg['shape-rendering'] = 'geometricPrecision'

start_x = freq_to_x(125)
end_x = freq_to_x(8000)
start_y = dB_to_y(-10)
end_y = dB_to_y(120)

tick_offset = 8
# Draw grid and labels from -10 to 120
for i in range(-10, 121, 10):
    y = dB_to_y(i)
    dwg.add(dwg.line(start=(start_x, y), end=(end_x, y), stroke='lightgray')) 
    # if i == 0:
    #     dx = -30
    #     dwg.add(dwg.text(f"0", insert=(start_x - tick_offset + dx, y+6), text_anchor="end", font_size="20px", font_family="FiraCode-Regular")) 
    #     dwg.add(dwg.text(f"dBHL", insert=(start_x - tick_offset + dx + 2, y+6), text_anchor="start", font_size="15px", font_family="FiraCode-Regular")) 
    # else:
    if i==-10:
        continue
    dwg.add(dwg.text(f"{i}", insert=(start_x - tick_offset, y+6), text_anchor="end", font_size="20px", font_family="FiraCode-Regular", fill=gray)) 

for gl in grid_lines:
    x = freq_to_x(gl)
    dwg.add(dwg.line(start=(x, start_y), end=(x, end_y), stroke='lightgray')) 
    if gl == 125:
        dwg.add(dwg.text("125 Hz", insert=(x+0, start_y - tick_offset),  text_anchor="middle", font_size="20px",  font_family="FiraCode-Regular", fill=gray))
        continue

    dwg.add(dwg.text(f"{gl}", insert=(x+0, start_y - tick_offset),  text_anchor="middle", font_size="20px",  font_family="FiraCode-Regular", fill=gray)) # transform=f"rotate(-45 {x-20} 560)",


# Draw axes
# dwg.add(dwg.line(start=(start_x, start_y), end=(start_x, end_y), stroke='black')) 
dwg.add(dwg.line(start=(start_x,dB_to_y(20)), end=(end_x,dB_to_y(20)), stroke='black', stroke_width=2)) 

# Add units per axis
unit_delta = 15
# dwg.add(dwg.text("Hz", insert=(freq_to_x(1000), end_y + unit_delta),  text_anchor="middle", font_size="20px", font_family="FiraCode-Regular")) 
dwg.add(dwg.text("dB HL", insert=(30 - 50, 270  + 35), font_size="15px", transform="rotate(-90 30 270)", font_family="FiraCode-Regular", fill=gray)) 






# Frequencies for dashed lines
dash_freqs = [750, 1500, 3000, 6000]

for freq in dash_freqs:
    x = freq_to_x(freq)
    dwg.add(dwg.line(start=(x, start_y), end=(x, end_y), stroke='lightgray', stroke_dasharray="5,5"))

# Draw lines between points before drawing the points to ensure the points appear on top
lines = []
points = []
last_point = None
side_length = 10
for data in audiogram_data:
    point = (freq_to_x(data['frequency']), dB_to_y(data['dB']))
    if last_point is not None:
        # lines.append(dwg.line(start=last_point, end=point, stroke='black', stroke_width=2))
        lines.append(dwg.line(start=last_point, end=point, stroke='black', stroke_width=2, stroke_dasharray="15,15"))
    # points.append(dwg.circle(center=point, r=4, fill='white', stroke='red'))
    points.append(dwg.rect(insert=(point[0]-side_length/2, point[1]-side_length/2), size=(side_length,side_length), fill='white', stroke='red')) 
    last_point = point

for line in lines:
    dwg.add(line)

for point in points:
    dwg.add(point)

# Save SVG
dwg.save()


# Convert to PDF
cairosvg.svg2pdf(
    file_obj=open("audiogram.svg", "rb"),  
    write_to="output5.pdf", 
    # input_width=1050,
    # input_height=876,
    output_width=2550,
    output_height=3300,
    # scale=10.0,
    dpi=300
)
