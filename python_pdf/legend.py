
import svgwrite
import numpy as np
import cairosvg
from math import cos, sin, pi, radians
import config



def add_square(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round', 'fill': 'white',  'stroke_width': 2}

    # Scaling
    s /= 2.0

    # Draws the square
    dwg.add(dwg.rect(insert=(pos_x - s, pos_y - s), size=(s*2, s*2), **stroke_attributes))

    return dwg


def add_circle(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'fill': 'white',  'stroke_width': 2}

    # Scaling
    radius = s * 0.5

    # Draws the circle
    dwg.add(dwg.circle(center=(pos_x, pos_y), r=radius, **stroke_attributes))

    return dwg


def add_triangle(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round', 'fill': 'white',  'stroke_width': 2}

    # Scaling
    s *= 0.6

    # Draws the triangle
    dwg.add(dwg.path(d=("M" + str(pos_x) + " " + str(pos_y-s*0.9) +
                        "L" + str(pos_x+s) + " " + str(pos_y+s*0.9) +
                        "L" + str(pos_x-s) + " " + str(pos_y+s*0.9) + "Z"),
                     **stroke_attributes))

    return dwg


def add_bottom_left_arrow(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"] 
    pos_y = pos["y"] + s/4

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Scaling
    s *= 0.7
    arrow_len = s * 1.2
    tail_len = 0.55 * s
    oy = s * 0.6

    # Draws the arrow
    dwg.add(dwg.path(d=("M" + str(pos_x) + " " + str(pos_y+oy) +
                        "L" + str(pos_x-arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "M" + str(pos_x-arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "L" + str(pos_x-arrow_len+tail_len) + " " + str(pos_y+arrow_len+oy) +
                        "M" + str(pos_x-arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "L" + str(pos_x-arrow_len) + " " + str(pos_y+arrow_len-tail_len+oy)),
                     **stroke_attributes))

    return dwg


def add_bottom_right_arrow(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"] + s/4

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Scaling
    s *= 0.7
    arrow_len = s * 1.2
    tail_len = 0.55 * s
    oy = s * 0.6

    # Draws the arrow
    dwg.add(dwg.path(d=("M" + str(pos_x) + " " + str(pos_y+oy) +
                        "L" + str(pos_x+arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "M" + str(pos_x+arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "L" + str(pos_x+arrow_len-tail_len) + " " + str(pos_y+arrow_len+oy) +
                        "M" + str(pos_x+arrow_len) + " " + str(pos_y+arrow_len+oy) +
                        "L" + str(pos_x+arrow_len) + " " + str(pos_y+arrow_len-tail_len+oy)),
                     **stroke_attributes))

    return dwg


def add_left_bracket(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Scaling
    s *= 0.4
    v = s * 3  # Increase this factor for longer brackets

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2, 'fill': 'none'}

    # Draws the left bracket shape
    dwg.add(dwg.line(start=(pos_x, pos_y - v / 2), end=(pos_x - s, pos_y - v / 2), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x - s, pos_y - v / 2), end=(pos_x - s, pos_y + v / 2), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x - s, pos_y + v / 2), end=(pos_x, pos_y + v / 2), **stroke_attributes))
    return dwg


def add_right_bracket(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Scaling
    s *= 0.4
    v = s * 3  # Increase this factor for longer brackets

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2, 'fill': 'none'}

    # Draws the right bracket shape
    dwg.add(dwg.line(start=(pos_x, pos_y - v / 2), end=(pos_x + s, pos_y - v / 2), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x + s, pos_y - v / 2), end=(pos_x + s, pos_y + v / 2), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x + s, pos_y + v / 2), end=(pos_x, pos_y + v / 2), **stroke_attributes))
    return dwg



def add_x(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Scaling
    s *= 0.5

    # Draws the "x" shape
    dwg.add(dwg.path(d=("M" + str(pos_x+s) + " " + str(pos_y-s) +
                        "L" + str(pos_x-s) + " " + str(pos_y+s) +
                        "M" + str(pos_x-s) + " " + str(pos_y-s) +
                        "L" + str(pos_x+s) + " " + str(pos_y+s)),
                     **stroke_attributes))

    return dwg



def add_greater_than(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Scaling
    s *= 0.5

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Draws the ">"
    dwg.add(dwg.line(start=(pos_x - s, pos_y - s),
                     end=(pos_x + s, pos_y), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x + s, pos_y),
                     end=(pos_x - s, pos_y + s), **stroke_attributes))

    return dwg


def add_less_than(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Scaling
    s *= 0.5

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Draws the "<"
    dwg.add(dwg.line(start=(pos_x + s, pos_y - s),
                     end=(pos_x - s, pos_y), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x - s, pos_y),
                     end=(pos_x + s, pos_y + s), **stroke_attributes))

    return dwg


def add_u(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2, 'fill': 'none'}

    # Scaling
    s *= 0.5

    # Draws the "U" shape
    dwg.add(dwg.path(d=("M" + str(pos_x-s) + " " + str(pos_y) +
                        "A" + str(s) + " " + str(s) + " 0 1 0 " + str(pos_x+s) + " " + str(pos_y) +
                        "L" + str(pos_x+s) + " " + str(pos_y-s*1.3) +
                        "M" + str(pos_x-s) + " " + str(pos_y) +
                        "L" + str(pos_x-s) + " " + str(pos_y-s*1.3)),
                     **stroke_attributes))

    return dwg


def add_s(dwg, r, pos, stroke_color=config.gray):

    scale = 0.3
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"] + r * scale
    r=r*scale
    # r = 10  # Radius is increased for a more recognizable "S"
    v_y = -r * 2  # Displacement vector adjusted

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Extended line at the top
    dwg.add(dwg.line(start=(pos_x + r / 2, pos_y - r + v_y), end=(pos_x , pos_y - r + v_y), **stroke_attributes))

    # Arcs
    arc1_center_x = pos_x
    arc1_center_y = pos_y + v_y
    arc1_start_angle = pi / 2
    arc1_end_angle = pi / 2 * 3

    arc2_center_x = pos_x
    arc2_center_y = pos_y + r * 2 + v_y
    arc2_start_angle = -pi / 2
    arc2_end_angle = pi / 2

    dwg.add(dwg.path(d=f"M {arc1_center_x + r * cos(arc1_start_angle)} {arc1_center_y + r * sin(arc1_start_angle)} "
                    f"A {r} {r} 0 0 1 {arc1_center_x + r * cos(arc1_end_angle)} {arc1_center_y + r * sin(arc1_end_angle)}",
                     fill='none', **stroke_attributes))

    dwg.add(dwg.path(d=f"M {arc2_center_x + r * cos(arc2_start_angle)} {arc2_center_y + r * sin(arc2_start_angle)} "
                    f"A {r} {r} 0 0 1 {arc2_center_x + r * cos(arc2_end_angle)} {arc2_center_y + r * sin(arc2_end_angle)}",
                     fill='none', **stroke_attributes))

    # Extended line at the bottom
    dwg.add(dwg.line(start=(pos_x, pos_y + 3 * r + v_y), end=(pos_x - r * 0.7, pos_y + 3 * r + v_y), **stroke_attributes))

    return dwg






def add_z(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    # s = 50  # Size for a more recognizable "Z"
    a = 0.75

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Moves the "pen" to the starting position
    start_point = (pos_x - s * a, pos_y + s)

    # Draws lines in sequence
    dwg.add(dwg.line(start=start_point,
                    end=(pos_x + s * a, pos_y + s), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x + s * a, pos_y + s),
                    end=(pos_x - s * a, pos_y - s), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x - s * a, pos_y - s),
                    end=(pos_x + s * a, pos_y - s), **stroke_attributes))
    
    return dwg

def add_a(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"]
    pos_y = pos["y"]

    s = s*0.6
    # s = 50  # Size for a more recognizable "A"
    a = 0.75

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Moves the "pen" to the starting position
    start_point = (pos_x - s * a, pos_y + s)

    # Draws lines in sequence to form the letter "A"
    dwg.add(dwg.line(start=start_point,
                     end=(pos_x, pos_y - s), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x, pos_y - s),
                     end=(pos_x + s * a, pos_y + s), **stroke_attributes))

    # Draws the crossbar
    dwg.add(dwg.line(start=(pos_x - s * a / 2.0, pos_y + s * 0.2),
                     end=(pos_x + s * a / 2.0, pos_y + s * 0.2), **stroke_attributes))
    
    return dwg



def add_asterisk(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"] - s * 0.8
    pos_y = pos["y"] - s * 0.8

    # s = 50  # Size for a more recognizable asterisk
    s *= 0.4

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Defines the rotation angle
    theta = radians(30.0)

    # Calculates the displacement vectors
    x = [0., s]
    y = [s, 0.]

    # Draws the first line of the asterisk
    start_point = (pos_x + x[0] * cos(theta) + y[0] * sin(theta),
                   pos_y + x[1] * cos(theta) + y[1] * sin(theta))
    end_point = (pos_x - x[0] * cos(theta) - y[0] * sin(theta),
                 pos_y - x[1] * cos(theta) - y[1] * sin(theta))
    dwg.add(dwg.line(start=start_point, end=end_point, **stroke_attributes))

    # Draws the second line of the asterisk
    start_point = (pos_x - x[0] * cos(theta) + y[0] * sin(theta),
                   pos_y - x[1] * cos(theta) + y[1] * sin(theta))
    end_point = (pos_x + x[0] * cos(theta) - y[0] * sin(theta),
                 pos_y + x[1] * cos(theta) - y[1] * sin(theta))
    dwg.add(dwg.line(start=start_point, end=end_point, **stroke_attributes))

    # Draws the third line of the asterisk
    dwg.add(dwg.line(start=(pos_x + y[0], pos_y + y[1]),
                     end=(pos_x - y[0], pos_y - y[1]), **stroke_attributes))
    
    return dwg

def add_vt(dwg, s, pos, stroke_color=config.gray):
    # Initial position
    pos_x = pos["x"] + s
    pos_y = pos["y"] - s/2

    # Scaling
    s *= 0.5
    xt = s / 3.0

    # Stroke attributes
    stroke_attributes = {'stroke': stroke_color, 'stroke-linecap': 'round',  'stroke_width': 2}

    # Draws the "V"
    dwg.add(dwg.line(start=(pos_x - s / 2.0, pos_y),
                     end=(pos_x, pos_y - s), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x - s / 2.0, pos_y),
                     end=(pos_x - s, pos_y - s), **stroke_attributes))

    # Draws the "T"
    dwg.add(dwg.line(start=(pos_x + s / 2.0 + xt, pos_y),
                     end=(pos_x + s / 2.0 + xt, pos_y - s), **stroke_attributes))
    dwg.add(dwg.line(start=(pos_x + xt, pos_y - s),
                     end=(pos_x + s + xt, pos_y - s), **stroke_attributes))

    return dwg


from svgwrite import Drawing, rgb

class Legend:
    def __init__(self):
        self.space = 10
        self.ss = 10
        self.stroke_width = 2
        self.stroke_style = 'solid'
        self.LEGEND_SYMBOL_STROKE_COLOR = rgb(0, 0, 0)
        self.LEGEND_TEXT_COLOR = rgb(0, 0, 0)
        self.LEGEND_TITLES_COLOR = rgb(0, 0, 255)
        self.GRAY = rgb(128, 128, 128)
        self.LEGEND_BORDER_COLOR = rgb(255, 0, 0)

    # def draw_text(self, dwg, content, x, y, text_anchor):
    #     dwg.add(dwg.text(content, insert=(x, y), font_size="16px", text_anchor=text_anchor, font_family="Lato, Fira Code, sans-serif", fill=config.gray))

    def draw_text(self, dwg, content, x, y, text_anchor):
        lines = content.split('\n')
        for i, line in enumerate(lines):
            dy = i * 20  # Adjust as needed
            dwg.add(dwg.text(line, insert=(x, y + dy), font_size="16px", text_anchor=text_anchor, font_family="Lato, Fira Code, sans-serif", fill=config.gray))


    def draw_legend(self, dwg, bounds, top_left, unused_symbols=None):
        bounds_width = bounds["width"]
        bounds_height = bounds["height"]
        top = top_left["top"]
        left = top_left["left"]

        symbol_x_offset = 15
        text_y_offset = 4
        left_symbol = left + self.space + symbol_x_offset
        right_symbol = left + bounds_width - self.space - symbol_x_offset

        v = 15.0 + top
        self.draw_text(dwg, "DROITE", left + self.space + 4.0, v, 'start')
        self.draw_text(dwg, "GAUCHE", left + bounds_width - self.space - 4.0, v, 'end')

        v += 1 * self.ss - 1.0
        dwg.add(dwg.rect(insert=(left, v), size=(bounds_width, 3 * self.ss - 1.0), fill=config.super_light_gray, stroke='none'))
        v += 2 * self.ss - 1.0
        self.draw_text(dwg, "SEUIL AÉRIEN", left + bounds_width / 2.0, v, "middle")
        
        # dwg.add(dwg.text("SEUIL AÉRIEN", insert=(left + bounds_width / 2.0, v), font_size="16px", text_anchor=text_anchor, font_family="Lato, Fira Code, sans-serif", fill=config.gray, text_anchor = 'middle'))

        lines = [
            ("Non masqué", add_circle, add_x),
            ("Masqué", add_square, add_triangle),
            ("Inconfort", add_u, add_u),
            ("Champ libre", add_s, add_s),
            ("Avec appareil auditif", add_a, add_a),
        ]

        for line in lines:
            text, left_func, right_func = line
            if unused_symbols is not None and text in unused_symbols:
                continue
            v += 2 * self.ss - 1.0
            self.draw_text(dwg, text, left + bounds_width / 2.0, v + text_y_offset, "middle")
            left_func(dwg, self.ss, {"x": left_symbol, "y": v})
            right_func(dwg, self.ss, {"x": right_symbol, "y": v})


        lines = [
            ("Non masqué ", add_less_than, add_greater_than),
            ("Masqué ", add_left_bracket, add_right_bracket),
        ]


        v += 1 * self.ss - 1.0
        dwg.add(dwg.rect(insert=(left, v), size=(bounds_width, 3 * self.ss - 1.0), fill=config.super_light_gray, stroke='none'))
        v += 2 * self.ss - 1.0
        self.draw_text(dwg, "SEUIL OSSEUX", left + bounds_width / 2.0, v, "middle")

        for line in lines:
            text, left_func, right_func = line
            if unused_symbols is not None and text in unused_symbols:
                continue

            v += 2 * self.ss - 1.0
            self.draw_text(dwg, text, left + bounds_width / 2.0, v + text_y_offset, "middle")
            left_func(dwg, self.ss, {"x": left_symbol, "y": v})
            right_func(dwg, self.ss, {"x": right_symbol, "y": v})




        v += 0.5 * self.ss 

        v += 1 * self.ss - 1.0
        dwg.add(dwg.rect(insert=(left, v), size=(bounds_width, 1 * self.ss - 1.0), fill=config.super_light_gray, stroke='none'))
        v += 2 * self.ss - 1.0


        if unused_symbols is not None and "Pas de réponse" not in unused_symbols:
            upper_shift = self.ss
            self.draw_text(dwg, "Pas de réponse", left+ bounds_width / 2.0, v + 4, "middle")
            add_bottom_left_arrow(dwg, self.ss, {"x": left_symbol + 5, "y": v - upper_shift})
            add_bottom_right_arrow( dwg, self.ss, {"x": right_symbol - 5, "y": v  - upper_shift})
            v += 2 * self.ss - 1.0

        if unused_symbols is not None and "Vibrotactile" not in unused_symbols:
            lower_shift = self.ss / 2 + 2
            self.draw_text(dwg, "Vibrotactile",left+ bounds_width / 2.0, v + 4, "middle")
            add_vt(dwg, self.ss, {"x": right_symbol - self.ss, "y": v + lower_shift})
            add_vt(dwg, self.ss, {"x": left_symbol - self.ss, "y": v + lower_shift})
            v += 2 * self.ss - 1.0

        if unused_symbols is not None and "Surassourdissement ou\nmasque insuffisant" not in unused_symbols:
            self.draw_text(dwg, "Surassourdissement ou\nmasque insuffisant", left+ bounds_width / 2.0, v + 4, "middle")
            add_asterisk(dwg, self.ss, {"x": right_symbol + self.ss * 0.8, "y": v + self.ss * 2})
            add_asterisk(dwg, self.ss, {"x": left_symbol + self.ss * 0.8, "y": v + self.ss * 2})
            v += 4 * self.ss - 1.0

        height = v - top - 5

        dwg.add(dwg.rect(insert=(left, top), size=(bounds_width, height), rx=10, ry=10, fill='none', stroke=config.gray, stroke_width=1.5))



        return dwg
        