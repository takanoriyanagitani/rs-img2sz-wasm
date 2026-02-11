iwasi=./rs_img2sz_wasm.wasm

wasm-opt \
	-Oz \
	-o opt.wasm \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	"${iwasi}"
