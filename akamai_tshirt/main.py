"""Tesseract OCR with OpenCV and Python."""

import cv2
import numpy as np

# import easyocr
import pytesseract

img_path = "akamai_tshirt.jpg"
img = cv2.imread(img_path, cv2.IMREAD_UNCHANGED)

# Define the scaling factor or the desired dimensions
scale_factor = 5

# Scale up the image to get better resolution for OCR.
# Interpolation method doesn't really seem to matter much.
img = cv2.resize(
    img,
    (
        img.shape[1] * scale_factor,
        img.shape[0] * scale_factor,
    ),
    interpolation=cv2.INTER_LANCZOS4,
)

# extract the RED channel to extract the characters with orange highlighted
# background.
(_, _, img) = cv2.split(img)

################

# Normalize the image and attempt to enhance the text.
norm_img = np.zeros((img.shape[0], img.shape[1]))
img = cv2.normalize(img, norm_img, 0, 255, cv2.NORM_MINMAX)
img = cv2.threshold(img, 210, 255, cv2.THRESH_BINARY)[1]

# Blur and then sharpen the image to smooth out text.
img = cv2.GaussianBlur(img, (3, 3), 0)
kernel = np.array([[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]])
img = cv2.filter2D(img, -1, kernel)

# Attempt to remove noisy artifacts and separate characters that are connected
# to each other.
img = cv2.dilate(img, np.ones((6, 6), np.uint8), iterations=1)
img = cv2.erode(img, np.ones((3, 3), np.uint8), iterations=1)

# Write out the image for debugging.
cv2.imwrite("cleaned.jpg", img)

################

# reader = easyocr.Reader(
#     ["en"]
# )  # this needs to run only once to load the model into memory
# result = reader.readtext("cleaned.jpg")

# for r in result:
#     print(r[1])

# print("#" * 20)

text = pytesseract.image_to_string(img, lang="eng")
text = text[text.index("IyE") : text.rindex("=") + 1]
text = text.replace(" ", "").replace("@", "0").replace("¥", "Y")
print(text)
