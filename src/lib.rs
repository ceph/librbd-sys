//! Low level bindings for Ceph's RBD interface.
//! Please see [Ceph librbd](http://docs.ceph.com/docs/master/rbd/librbdpy/#module-rbd) for more
//! information. Those are the python docs but the parameters should be the same.
#![allow(non_camel_case_types)]
extern crate libc;
extern crate librados_sys;

use self::libc::{int64_t, size_t, ssize_t, uint8_t, uint64_t};
use self::librados_sys::rados_ioctx_t;

pub type rbd_snap_t = *mut ::std::os::raw::c_void;
pub type rbd_image_t = *mut ::std::os::raw::c_void;
pub type librbd_progress_fn_t =
    ::std::option::Option<unsafe extern "C" fn(offset: uint64_t,
                                               total: uint64_t,
                                               ptr:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct rbd_snap_info_t{
    pub id: uint64_t,
    pub size: uint64_t,
    pub name: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for rbd_snap_info_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for rbd_snap_info_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct rbd_image_info_t {
    pub size: uint64_t,
    pub obj_size: uint64_t,
    pub num_objs: uint64_t,
    pub order: ::std::os::raw::c_int,
    pub block_name_prefix: [::std::os::raw::c_char; 24usize],
    pub parent_pool: int64_t,
    pub parent_name: [::std::os::raw::c_char; 96usize],
}
impl ::std::clone::Clone for rbd_image_info_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for rbd_image_info_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type rbd_completion_t = *mut ::std::os::raw::c_void;
pub type rbd_callback_t =
    ::std::option::Option<unsafe extern "C" fn(cb: rbd_completion_t,
                                               arg:
                                                   *mut ::std::os::raw::c_void)>;
#[link(name = "rbd")]
extern "C" {
    pub fn rbd_version(major: *mut ::std::os::raw::c_int,
                       minor: *mut ::std::os::raw::c_int,
                       extra: *mut ::std::os::raw::c_int);
    pub fn rbd_list(io: rados_ioctx_t, names: *mut ::std::os::raw::c_char,
                    size: *mut size_t) -> ::std::os::raw::c_int;
    pub fn rbd_create(io: rados_ioctx_t, name: *const ::std::os::raw::c_char,
                      size: uint64_t, order: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn rbd_create2(io: rados_ioctx_t, name: *const ::std::os::raw::c_char,
                       size: uint64_t, features: uint64_t,
                       order: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;

    /// create new rbd image
    ///
    /// The stripe_unit must be a factor of the object size (1 << order).
    /// The stripe_count can be one (no intra-object striping) or greater
    /// than one.  The RBD_FEATURE_STRIPINGV2 must be specified if the
    /// stripe_unit != the object size and the stripe_count is != 1.
    ///
    /// # Arguments
    /// @param io ioctx
    /// @param name image name
    /// @param size image size in bytes
    /// @param features initial feature bits
    /// @param order object/block size, as a power of two (object size == 1 << order)
    /// @param stripe_unit stripe unit size, in bytes.
    /// @param stripe_count number of objects to stripe over before looping
    /// @return 0 on success, or negative error code
    pub fn rbd_create3(io: rados_ioctx_t, name: *const ::std::os::raw::c_char,
                       size: uint64_t, features: uint64_t,
                       order: *mut ::std::os::raw::c_int,
                       stripe_unit: uint64_t, stripe_count: uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_clone(p_ioctx: rados_ioctx_t,
                     p_name: *const ::std::os::raw::c_char,
                     p_snapname: *const ::std::os::raw::c_char,
                     c_ioctx: rados_ioctx_t,
                     c_name: *const ::std::os::raw::c_char,
                     features: uint64_t, c_order: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn rbd_clone2(p_ioctx: rados_ioctx_t,
                      p_name: *const ::std::os::raw::c_char,
                      p_snapname: *const ::std::os::raw::c_char,
                      c_ioctx: rados_ioctx_t,
                      c_name: *const ::std::os::raw::c_char,
                      features: uint64_t, c_order: *mut ::std::os::raw::c_int,
                      stripe_unit: uint64_t,
                      stripe_count: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn rbd_remove(io: rados_ioctx_t, name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_remove_with_progress(io: rados_ioctx_t,
                                    name: *const ::std::os::raw::c_char,
                                    cb: librbd_progress_fn_t,
                                    cbdata: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn rbd_rename(src_io_ctx: rados_ioctx_t,
                      srcname: *const ::std::os::raw::c_char,
                      destname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_open(io: rados_ioctx_t, name: *const ::std::os::raw::c_char,
                    image: *mut rbd_image_t,
                    snap_name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    /// Open an image in read-only mode.
    ///
    /// This is intended for use by clients that cannot write to a block
    /// device due to cephx restrictions. There will be no watch
    /// established on the header object, since a watch is a write. This
    /// means the metadata reported about this image (parents, snapshots,
    /// size, etc.) may become stale. This should not be used for
    /// long-running operations, unless you can be sure that one of these
    /// properties changing is safe.
    ///
    /// Attempting to write to a read-only image will return -EROFS.
    ///
    /// # Arguments
    /// @param io ioctx to determine the pool the image is in
    /// @param name image name
    /// @param image where to store newly opened image handle
    /// @param snap_name name of snapshot to open at, or NULL for no snapshot
    /// @returns 0 on success, negative error code on failure
    pub fn rbd_open_read_only(io: rados_ioctx_t,
                              name: *const ::std::os::raw::c_char,
                              image: *mut rbd_image_t,
                              snap_name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_close(image: rbd_image_t) -> ::std::os::raw::c_int;
    pub fn rbd_resize(image: rbd_image_t, size: uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_resize_with_progress(image: rbd_image_t, size: uint64_t,
                                    cb: librbd_progress_fn_t,
                                    cbdata: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn rbd_stat(image: rbd_image_t, info: *mut rbd_image_info_t,
                    infosize: size_t) -> ::std::os::raw::c_int;
    pub fn rbd_get_old_format(image: rbd_image_t, old: *mut uint8_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_size(image: rbd_image_t, size: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_features(image: rbd_image_t, features: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_stripe_unit(image: rbd_image_t, stripe_unit: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_stripe_count(image: rbd_image_t,
                                stripe_count: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_overlap(image: rbd_image_t, overlap: *mut uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_get_parent_info(image: rbd_image_t,
                               parent_poolname: *mut ::std::os::raw::c_char,
                               ppoolnamelen: size_t,
                               parent_name: *mut ::std::os::raw::c_char,
                               pnamelen: size_t,
                               parent_snapname: *mut ::std::os::raw::c_char,
                               psnapnamelen: size_t) -> ::std::os::raw::c_int;
    pub fn rbd_copy(image: rbd_image_t, dest_io_ctx: rados_ioctx_t,
                    destname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_copy2(src: rbd_image_t, dest: rbd_image_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_copy_with_progress(image: rbd_image_t, dest_p: rados_ioctx_t,
                                  destname: *const ::std::os::raw::c_char,
                                  cb: librbd_progress_fn_t,
                                  cbdata: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn rbd_copy_with_progress2(src: rbd_image_t, dest: rbd_image_t,
                                   cb: librbd_progress_fn_t,
                                   cbdata: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_list(image: rbd_image_t, snaps: *mut rbd_snap_info_t,
                         max_snaps: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_list_end(snaps: *mut rbd_snap_info_t);
    pub fn rbd_snap_create(image: rbd_image_t,
                           snapname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_remove(image: rbd_image_t,
                           snapname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_rollback(image: rbd_image_t,
                             snapname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_rollback_with_progress(image: rbd_image_t,
                                           snapname:
                                               *const ::std::os::raw::c_char,
                                           cb: librbd_progress_fn_t,
                                           cbdata:
                                               *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;

    /// Prevent a snapshot from being deleted until it is unprotected.
    pub fn rbd_snap_protect(image: rbd_image_t,
                            snap_name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    /// Allow a snaphshot to be deleted
    pub fn rbd_snap_unprotect(image: rbd_image_t,
                              snap_name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    /// Determine whether a snapshot is protected.
    pub fn rbd_snap_is_protected(image: rbd_image_t,
                                 snap_name: *const ::std::os::raw::c_char,
                                 is_protected: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn rbd_snap_set(image: rbd_image_t,
                        snapname: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_flatten(image: rbd_image_t) -> ::std::os::raw::c_int;

    /// List all images that are cloned from the image at the
    /// snapshot that is set via rbd_snap_set().
    ///
    /// This iterates over all pools, so it should be run by a user with
    /// read access to all of them. pools_len and images_len are filled in
    /// with the number of bytes put into the pools and images buffers.
    /// If the provided buffers are too short, the required lengths are
    /// still filled in, but the data is not and -ERANGE is returned.
    /// Otherwise, the buffers are filled with the pool and image names
    /// of the children, with a '\0' after each.
    /// # Arguments
    /// @param image which image (and implicitly snapshot) to list clones of
    /// @param pools buffer in which to store pool names
    /// @param pools_len number of bytes in pools buffer
    /// @param images buffer in which to store image names
    /// @param images_len number of bytes in images buffer
    /// @returns number of children on success, negative error code on failure
    /// @returns -ERANGE if either buffer is too short
    pub fn rbd_list_children(image: rbd_image_t,
                             pools: *mut ::std::os::raw::c_char,
                             pools_len: *mut size_t,
                             images: *mut ::std::os::raw::c_char,
                             images_len: *mut size_t) -> ssize_t;

    /// List clients that have locked the image and information about the lock.
    ///
    /// The number of bytes required in each buffer is put in the
    /// corresponding size out parameter. If any of the provided buffers
    /// are too short, -ERANGE is returned after these sizes are filled in.
    /// # Arguments
    /// @param exclusive where to store whether the lock is exclusive (1) or shared (0)
    /// @param tag where to store the tag associated with the image
    /// @param tag_len number of bytes in tag buffer
    /// @param clients buffer in which locker clients are stored, separated by '\0'
    /// @param clients_len number of bytes in the clients buffer
    /// @param cookies buffer in which locker cookies are stored, separated by '\0'
    /// @param cookies_len number of bytes in the cookies buffer
    /// @param addrs buffer in which locker addresses are stored, separated by '\0'
    /// @param addrs_len number of bytes in the clients buffer
    /// @returns number of lockers on success, negative error code on failure
    /// @returns -ERANGE if any of the buffers are too short
    pub fn rbd_list_lockers(image: rbd_image_t,
                            exclusive: *mut ::std::os::raw::c_int,
                            tag: *mut ::std::os::raw::c_char,
                            tag_len: *mut size_t,
                            clients: *mut ::std::os::raw::c_char,
                            clients_len: *mut size_t,
                            cookies: *mut ::std::os::raw::c_char,
                            cookies_len: *mut size_t,
                            addrs: *mut ::std::os::raw::c_char,
                            addrs_len: *mut size_t) -> ssize_t;
    /// Take an exclusive lock on the image.
    /// # Arguments
    /// @param image the image to lock
    /// @param cookie user-defined identifier for this instance of the lock
    /// @returns 0 on success, negative error code on failure
    /// @returns -EBUSY if the lock is already held by another (client, cookie) pair
    /// @returns -EEXIST if the lock is already held by the same (client, cookie) pair
    pub fn rbd_lock_exclusive(image: rbd_image_t,
                              cookie: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    /// Take a shared lock on the image.
    ///
    /// Other clients may also take a shared lock, as lock as they use the
    /// same tag.
    ///
    /// # Arguments
    /// @param image the image to lock
    /// @param cookie user-defined identifier for this instance of the lock
    /// @param tag user-defined identifier for this shared use of the lock
    /// @returns 0 on success, negative error code on failure
    /// @returns -EBUSY if the lock is already held by another (client, cookie) pair
    /// @returns -EEXIST if the lock is already held by the same (client, cookie) pair
    pub fn rbd_lock_shared(image: rbd_image_t,
                           cookie: *const ::std::os::raw::c_char,
                           tag: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    /// Release a shared or exclusive lock on the image.
    ///
    /// # Arguments
    /// @param image the image to unlock
    /// @param cookie user-defined identifier for the instance of the lock
    /// @returns 0 on success, negative error code on failure
    /// @returns -ENOENT if the lock is not held by the specified (client, cookie) pair
    pub fn rbd_unlock(image: rbd_image_t,
                      cookie: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;

    /// Release a shared or exclusive lock that was taken by the specified client.
    ///
    /// # Arguments
    /// image the image to unlock
    /// client the entity holding the lock (as given by rbd_list_lockers())
    /// cookie user-defined identifier for the instance of the lock to break
    ///
    /// @returns 0 on success, negative error code on failure
    /// @returns -ENOENT if the lock is not held by the specified (client, cookie) pair
    pub fn rbd_break_lock(image: rbd_image_t,
                          client: *const ::std::os::raw::c_char,
                          cookie: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn rbd_read(image: rbd_image_t, ofs: uint64_t, len: size_t,
                    buf: *mut ::std::os::raw::c_char) -> ssize_t;

    /// iterate read over an image
    /// Reads each region of the image and calls the callback.  If the
    /// buffer pointer passed to the callback is NULL, the given extent is
    /// defined to be zeros (a hole).  Normally the granularity for the
    /// callback is the image stripe size.
    /// # Arguments
    /// @param image image to read
    /// @param ofs offset to start from
    /// @param len bytes of source image to cover
    /// @param cb callback for each region
    /// @returns 0 success, error otherwise
    pub fn rbd_read_iterate2(image: rbd_image_t, ofs: uint64_t, len: uint64_t,
                             cb:
                                 ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                uint64_t,
                                                                            arg2:
                                                                                size_t,
                                                                            arg3:
                                                                                *const ::std::os::raw::c_char,
                                                                            arg4:
                                                                                *mut ::std::os::raw::c_void)
                                                           ->
                                                               ::std::os::raw::c_int>,
                             arg: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;

    /// get difference between two versions of an image
    ///
    /// This will return the differences between two versions of an image
    /// via a callback, which gets the offset and length and a flag
    /// indicating whether the extent exists (1), or is known/defined to
    /// be zeros (a hole, 0).  If the source snapshot name is NULL, we
    /// interpret that as the beginning of time and return all allocated
    /// regions of the image.  The end version is whatever is currently
    /// selected for the image handle (either a snapshot or the writeable
    /// head).
    /// # Arguments
    /// @param fromsnapname start snapshot name, or NULL
    /// @param ofs start offset
    /// @param len len in bytes of region to report on
    /// @param cb callback to call for each allocated region
    /// @param arg argument to pass to the callback
    /// @returns 0 on success, or negative error code on error
    pub fn rbd_diff_iterate(image: rbd_image_t,
                            fromsnapname: *const ::std::os::raw::c_char,
                            ofs: uint64_t, len: uint64_t,
                            cb:
                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                               uint64_t,
                                                                           arg2:
                                                                               size_t,
                                                                           arg3:
                                                                               ::std::os::raw::c_int,
                                                                           arg4:
                                                                               *mut ::std::os::raw::c_void)
                                                          ->
                                                              ::std::os::raw::c_int>,
                            arg: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
    pub fn rbd_write(image: rbd_image_t, ofs: uint64_t, len: size_t,
                     buf: *const ::std::os::raw::c_char) -> ssize_t;
    pub fn rbd_discard(image: rbd_image_t, ofs: uint64_t, len: uint64_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_aio_write(image: rbd_image_t, off: uint64_t, len: size_t,
                         buf: *const ::std::os::raw::c_char,
                         c: rbd_completion_t) -> ::std::os::raw::c_int;
    pub fn rbd_aio_read(image: rbd_image_t, off: uint64_t, len: size_t,
                        buf: *mut ::std::os::raw::c_char, c: rbd_completion_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_aio_discard(image: rbd_image_t, off: uint64_t, len: uint64_t,
                           c: rbd_completion_t) -> ::std::os::raw::c_int;
    pub fn rbd_aio_create_completion(cb_arg: *mut ::std::os::raw::c_void,
                                     complete_cb: rbd_callback_t,
                                     c: *mut rbd_completion_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_aio_is_complete(c: rbd_completion_t) -> ::std::os::raw::c_int;
    pub fn rbd_aio_wait_for_complete(c: rbd_completion_t)
     -> ::std::os::raw::c_int;
    pub fn rbd_aio_get_return_value(c: rbd_completion_t) -> ssize_t;
    pub fn rbd_aio_release(c: rbd_completion_t);

    /// Start a flush if caching is enabled. Get a callback when
    /// the currently pending writes are on disk.
    /// # Arguments
    /// @param image the image to flush writes to
    /// @param c what to call when flushing is complete
    /// @returns 0 on success, negative error code on failure
    pub fn rbd_flush(image: rbd_image_t) -> ::std::os::raw::c_int;
    pub fn rbd_aio_flush(image: rbd_image_t, c: rbd_completion_t)
     -> ::std::os::raw::c_int;

    /// Drop any cached data for an image
    ///
    /// # Arguments
    /// @param image the image to invalidate cached data for
    /// @returns 0 on success, negative error code on failure
    pub fn rbd_invalidate_cache(image: rbd_image_t) -> ::std::os::raw::c_int;
}
