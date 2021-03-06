#!/usr/bin/env ruby

require "../target/debug/liblibimagruby.so"

color = true
verbose = true
debug = true

RImag.init_logger debug, verbose, color

store_handle = RStoreHandle::new(false, "/tmp/store")
id = RStoreId::new_baseless("test")
test_handle = store_handle.create(id)

RImag.info "Created #{test_handle.location.to_str} from Ruby"

