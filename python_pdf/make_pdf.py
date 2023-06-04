# import platform
# import subprocess


# def activate_environment():
#     current_os = platform.system()

#     if current_os == 'Windows':
#         # Windows activation command
#         command = 'activate myenv'
#     elif current_os == 'Darwin':
#         # macOS activation command
#         command = 'source activate myenv'
#     else:
#         # Unsupported operating system
#         print("Unsupported operating system.")
#         return

#     subprocess.call(command, shell=True)


# # Call the function to activate the environment
# activate_environment()


from cairosvg import svg2pdf
from reportlab.pdfgen import canvas
from PyPDF2 import PdfWriter, PdfReader

############################################################
# Procedure for create a local environment, and running this code in Windows 10
# python - m venv env
# env\Scripts\activate
# pip install cairosvg reportlab pypdf2
# pip install pipwin
# pipwin install cairocffi
############################################################

# Convert SVG to PDF
with open("duck.svg", "r") as svg_file, open("duck.pdf", "wb") as pdf_file:
    svg2pdf(file_obj=svg_file, write_to=pdf_file)

# Create a PDF with a paragraph
c = canvas.Canvas("text.pdf")
c.setFont("Helvetica", 12)
# Adjust coordinates as per your requirements
textobject = c.beginText(40, 800)
lines = ["This is line 1",
         "This is line 2",
         "This is line 3"]
for line in lines:
    textobject.textLine(line)
c.drawText(textobject)
c.save()

# Merge the two PDFs
output = PdfWriter()
input1 = PdfReader(open("text.pdf", "rb"))
input2 = PdfReader(open("duck.pdf", "rb"))

page1 = input1.pages[0]
page2 = input2.pages[0]
page1.merge_page(page2)
output.add_page(page1)

# Write the output PDF
with open("output4.pdf", "wb") as outputStream:
    output.write(outputStream)
