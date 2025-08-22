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
	
	with open("pycamera2_frame", "r+b") as f:
		mm = mmap.mmap(f.fileno(),0)

		while True:
			mm.seek(0)
			a = mm.read_byte()
			mm.seek(0)
			if a == 255:
				a = 0
			else:
				a = a+1
			mm.write_byte(a)
			mm.seek(1)
			mm.write(capture_array)
			capture_array = picam2.capture_array("main")
		mm.close()
