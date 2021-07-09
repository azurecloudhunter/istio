LOCAL_REGISTRY_NAME='kind-registry'
LOCAL_REGISTRY_PORT='5000'
LOCAL_REGISTRY_RUNNING="$(docker inspect -f '{{.State.Running}}' "${LOCAL_REGISTRY_NAME}" 2>/dev/null || true)"
if [ "${LOCAL_REGISTRY_RUNNING}" != 'true' ]; then
  docker run \
    -d --restart=always -p "${LOCAL_REGISTRY_PORT}:5000" --name "${LOCAL_REGISTRY_NAME}" \
    registry:2
fi
