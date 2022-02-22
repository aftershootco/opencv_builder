use std::path::{Path, PathBuf};

#[cfg(feature = "imgproc")]
static IMGPROC: &str = "ON";
#[cfg(feature = "imgcodecs")]
static IMGCODECS: &str = "ON";

#[cfg(not(feature = "imgproc"))]
static IMGPROC: &str = "OFF";
#[cfg(not(feature = "imgcodecs"))]
static IMGCODECS: &str = "OFF";

#[cfg(feature = "static-link")]
static DYNAMIC_LINK: &str = "OFF";

#[cfg(not(feature = "static-link"))]
static DYNAMIC_LINK: &str = "ON";

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}

fn build_opencv<P: AsRef<Path>>(src_path: P) {
    #[cfg(feature = "build-opencv")]
    {
        cmake::Config::new(src_path)
            .define("BUILD_ZLIB", "OFF")
            .define("BUILD_JPEG", "OFF")
            .define("WITH_JPEG", "OFF")
            .define("WITH_PNG", "OFF")
            .define("BUILD_PNG", "OFF")
            .define("WITH_LAPACK", "OFF")
            .define("WITH_OPENCLAMDBLAS", "OFF")
            .define("WITH_OPENCLAMDFFT", "OFF")
            .define("WITH_IMGCODEC_HDR", "OFF")
            .define("WITH_IMGCODEC_PXM", "OFF")
            .define("BUILD_TIFF", "OFF")
            .define("BUILD_WEBP", "OFF")
            .define("BUILD_PROTOBUF", "OFF")
            .define("WITH_PROTOBUF", "OFF")
            .define("BUILD_TBB", "OFF")
            .define("WITH_TBB", "OFF")
            .define("WITH_1394", "OFF")
            .define("WITH_OPENGL", "OFF")
            .define("WITH_OPENCL", "OFF")
            .define("WITH_V4L", "OFF")
            .define("WITH_LIBV4L", "OFF")
            .define("WITH_GTK", "OFF")
            .define("WITH_GDAL", "OFF")
            .define("WITH_XINE", "OFF")
            .define("WITH_FFMPEG", "OFF")
            .define("BUILD_OPENEXR", "OFF")
            .define("OPENCV_GENERATE_PKGCONFIG", "OFF")
            .define("OPENCV_GENERATE_SETUPVARS", "OFF")
            .define("BUILD_opencv_cu.dabgsegm", "OFF")
            .define("BUILD_opencv_cu.dalegacy", "OFF")
            .define("BUILD_opencv_cu.dfilters", "OFF")
            .define("BUILD_opencv_cu.dastereo", "OFF")
            .define("BUILD_opencv_cudafeatures2d", "OFF")
            .define("BUILD_opencv_cudaoptflow", "OFF")
            .define("BUILD_opencv_cudacodec", "OFF")
            .define("BUILD_opencv_cudaimgproc", "OFF")
            .define("BUILD_opencv_cudawarping", "OFF")
            .define("BUILD_opencv_cudaarithm", "OFF")
            .define("BUILD_opencv_cudaobjdetect", "OFF")
            .define("BUILD_opencv_cudev", "OFF")
            .define("BUILD_opencv_superres", "OFF")
            .define("BUILD_opencv_ts", "OFF")
            .define("BUILD_opencv_videostab", "OFF")
            .define("BUILD_opencv_gapi", "OFF")
            .define("BUILD_opencv_apps", "OFF")
            .define("BUILD_opencv_world", "OFF")
            .define("INSTALL_C_EXAMPLES", "OFF")
            .define("BUILD_EXAMPLES", "OFF")
            .define("BUILD_PERF_TESTS", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("BUILD_DOCS", "OFF")
            .define("BUILD_opencv_python_bindings_generator", "OFF")
            .define("BUILD_opencv_python2", "OFF")
            .define("BUILD_opencv_python3", "OFF")
            .define("BUILD_opencv_java_bindings_generator", "OFF")
            .define("BUILD_IPP_IW", "OFF")
            .define("BUILD_JASPER", "OFF")
            .define("BUILD_opencv_stitching", "OFF")
            .define("BUILD_opencv_photo", "OFF")
            .define("BUILD_opencv_flann", "OFF")
            .define("BUILD_opencv_video", "OFF")
            .define("BUILD_opencv_videoio", "OFF")
            .define("BUILD_opencv_calib3d", "OFF")
            .define("BUILD_opencv_shape", "OFF")
            .define("BUILD_opencv_ml", "OFF")
            .define("BUILD_JAVA", "OFF")
            .define("BUILD_ITT", "OFF")
            .define("BUILD_PACKAGE", "OFF")
            .define("CPACK_BINARY_DEB", "OFF")
            .define("CPACK_BINARY_FREEBSD", "OFF")
            .define("CPACK_BINARY_IFW", "OFF")
            .define("CPACK_BINARY_NSIS", "OFF")
            .define("CPACK_BINARY_RPM", "OFF")
            .define("CPACK_BINARY_STGZ", "OFF")
            .define("CPACK_BINARY_TBZ2", "OFF")
            .define("CPACK_BINARY_TGZ", "OFF")
            .define("CPACK_BINARY_TXZ", "OFF")
            .define("CPACK_BINARY_TZ", "OFF")
            .define("CPACK_SOURCE_RPM", "OFF")
            .define("CPACK_SOURCE_TBZ2", "OFF")
            .define("CPACK_SOURCE_TGZ", "OFF")
            .define("CPACK_SOURCE_TXZ", "OFF")
            .define("CPACK_SOURCE_TZ", "OFF")
            .define("CPACK_SOURCE_ZIP", "OFF")
            .define("WITH_CUDA", "OFF")
            .define("WITH_GSTREAMER", "OFF")
            .define("WITH_GTK", "OFF")
            .define("WITH_IMGCODEC_SUNRASTER", "OFF")
            .define("WITH_IPP", "OFF")
            .define("WITH_ITT", "OFF")
            .define("WITH_JASPER", "OFF")
            .define("WITH_OPENEXR", "OFF")
            .define("WITH_PTHREADS_PF", "OFF")
            .define("WITH_QUIRC", "OFF")
            .define("WITH_TIFF", "OFF")
            .define("WITH_V4L", "OFF")
            .define("WITH_VTK", "OFF")
            .define("WITH_WEBP", "OFF")
            .define("ccitt", "OFF")
            .define("logluv", "OFF")
            .define("lzw", "OFF")
            .define("mdi", "OFF")
            .define("next", "OFF")
            .define("old_jpeg", "OFF")
            .define("opencv_dnn_PERF_CAFFE", "OFF")
            .define("opencv_dnn_PERF_CLCAFFE", "OFF")
            .define("packbits", "OFF")
            .define("thunder", "OFF")
            .define("BUILD_opencv_imgproc", IMGCODECS)
            .define("BUILD_opencv_imgcodecs", IMGPROC)
            .define("BUILD_opencv_highgui", "OFF")
            .define("BUILD_opencv_objdetect", "OFF")
            .define("BUILD_opencv_dnn", "OFF")
            .define("BUILD_opencv_features2d", "OFF")
            .define("BUILD_opencv_core", "ON")
            .define("BUILD_SHARED_LIBS", DYNAMIC_LINK)
            .profile("Release")
            .uses_cxx11()
            .build();
    }
}

fn main() {
    build_opencv(Path::new(".").join("opencv"));
    let out_dir = out_dir();
    let rustc_link = if DYNAMIC_LINK.eq("OFF") {
        "cargo:rustc-link-lib=static"
    } else {
        "cargo:rustc-link-lib"
    };
    #[cfg(target_os = "macos")]
    {
        println!(
            "cargo:rustc-link-search=native={}",
            out_dir.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            out_dir
                .join("lib")
                .join("opencv4")
                .join("3rdparty")
                .display()
        );
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("{}=libopenjp2", rustc_link);
        println!("{}=opencv_core", rustc_link);
        println!("{}=opencv_imgcodecs", rustc_link);
        println!("{}=opencv_imgproc", rustc_link);
        println!("{}=tegra_hal", rustc_link);
    }

    #[cfg(target_os = "linux")]
    {
        println!(
            "cargo:rustc-link-search=native={}",
            out_dir.join("lib").display()
        );
        println!("{}=opencv_core", rustc_link);
        println!("{}=opencv_imgcodecs", rustc_link);
        println!("{}=opencv_imgproc", rustc_link);
    }
}
