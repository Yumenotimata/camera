import time
import numpy as np
from picamera2 import Picamera2
import mmap


# メインプロセス
if __name__ == '__main__':
	picam2 = Picamera2()

	config_capture = picam2.create_still_configuration()
	picam2.configure(config_capture)

	picam2.start()

	time.sleep(2)

	# キャプチャする画像の情報を取得
	dims = picam2.camera_configuration()['main']['size']
	width, height = dims[0], dims[1]
	capture_array = picam2.capture_array("main")

	with open("pycamera2_frame", "wb") as f:
		f.close()
	
	with open("pycamera2_frame", "r+b") as f:
		mm = mmap.mmap(f.fileno(),0)
		mm.seek(0)
		mm.write(capture_array)
		mm.close()
