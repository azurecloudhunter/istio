set +o noclobber
chmod +x patch.sh
chmod +x kindlocalreg.sh
curl -L https://istio.io/downloadIstio | ISTIO_VERSION=1.10.2 TARGET_ARCH=x86_64 sh -
export PATH=/workspaces/istio/istio-1.10.2/bin:$PATH
