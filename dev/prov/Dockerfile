ARG BW_DEV_PROV_DOCKER_IMAGE
ARG BW_DEV_PROV_DOCKER_TAG
FROM ${BW_DEV_PROV_DOCKER_IMAGE}:${BW_DEV_PROV_DOCKER_TAG}

# RUN true && \
#     sudo apt-get update && \
#     sudo apt-get install -y \
#          dumb-init \
#         # libssl-dev \
#         # pkg-config \
#     && \
# true

RUN true && \
    apt-get update && \
    apt-get install -y \
         dumb-init \
        # libssl-dev \
        # pkg-config \
    && \
true

ENV PATH="/sh/:${PATH}"
