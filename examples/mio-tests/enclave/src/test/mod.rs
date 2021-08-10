mod waker;
mod close_on_drop;
mod events;
mod interest;
mod poll;
mod registering;
mod regressions;
mod tcp_listener;
mod tcp_socket;
mod tcp_stream;
mod tcp;
mod udp_socket;
mod unix_datagram;
mod unix_listener;
mod unix_stream;
mod util;

pub fn test_waker() {
    println!("Testing functions in waker inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    waker::is_send_and_sync();
    print!("Success.\n");

    print!("Testing waker()... ");
    waker::waker();
    print!("Success.\n");

    print!("Testing waker_multiple_wakeups_same_thread()... ");
    waker::waker_multiple_wakeups_same_thread();
    print!("Success.\n");


    print!("Testing waker_wakeup_different_thread()... ");
    waker::waker_wakeup_different_thread();
    print!("Success.\n");

    print!("Testing waker_multiple_wakeups_different_thread()... ");
    waker::waker_multiple_wakeups_different_thread();
    print!("Success.\n");

    println!("All tests passed in waker. \n");
}

pub fn test_close_on_drop() {
    println!("Testing functions in close_on_drop inside enclave.\n");

    print!("Testing close_on_drop()... ");
    close_on_drop::close_on_drop();
    print!("Success.\n");

    println!("All tests passed in close_on_drop. \n");

}

pub fn test_events() {
    println!("Testing functions in events inside enclave.\n");

    print!("Testing assert_event_source_implemented_for()... ");
    events::assert_event_source_implemented_for();
    print!("Success.\n");

    print!("Testing events_all()... ");
    events::events_all();
    print!("Success.\n");

    println!("All tests passed in events. \n");

}

pub fn test_interest() {
    println!("Testing functions in interest inside enclave.\n");

    print!("Testing is_tests()... ");
    interest::is_tests();
    print!("Success.\n");

    print!("Testing bit_or()... ");
    interest::bit_or();
    print!("Success.\n");

    print!("Testing fmt_debug()... ");
    interest::fmt_debug();
    print!("Success.\n");

    print!("Testing add()... ");
    interest::add();
    print!("Success.\n");

    println!("All tests passed in interest. \n");


}

pub fn test_poll() {
    println!("Testing functions in poll inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    poll::is_send_and_sync();
    print!("Success.\n");

    print!("Testing run_once_with_nothing()... ");
    poll::run_once_with_nothing();
    print!("Success.\n");

    print!("Testing add_then_drop()... ");
    poll::add_then_drop();
    print!("Success.\n");

    print!("Testing zero_duration_polls_events()... ");
    poll::zero_duration_polls_events();
    print!("Success.\n");

    print!("Testing poll_closes_fd()... ");
    poll::poll_closes_fd();
    print!("Success.\n");

    print!("Testing drop_cancels_interest_and_shuts_down()... ");
    poll::drop_cancels_interest_and_shuts_down();
    print!("Success.\n");

    print!("Testing registry_behind_arc()... ");
    poll::registry_behind_arc();
    print!("Success.\n");

    print!("Testing registry_operations_are_thread_safe()... ");
    poll::registry_operations_are_thread_safe();
    print!("Success.\n");

    print!("Testing register_during_poll()... ");
    poll::register_during_poll();
    print!("Success.\n");

    print!("Testing reregister_interest_token_usage()... ");
    poll::reregister_interest_token_usage();
    print!("Success.\n");

    print!("Testing poll_ok_after_cancelling_pending_ops()... ");
    poll::poll_ok_after_cancelling_pending_ops();
    print!("Success.\n");

    print!("Testing poll_registration()... ");
    poll::poll_registration();
    print!("Success.\n");

    print!("Testing poll_erroneous_registration()... ");
    poll::poll_erroneous_registration();
    print!("Success.\n");

    // Insert debug_assertions tests here

    #[cfg(debug_assertions)]
    {
        print!("Testing double_register_different_token()... ");
        poll::double_register_different_token();
        print!("Success.\n");
    }

    #[cfg(debug_assertions)]
    {
        print!("Testing reregister_without_register()... ");
        poll::reregister_without_register();
        print!("Success.\n");
    }

    #[cfg(debug_assertions)]
    {
        print!("Testing deregister_without_register()... ");
        poll::deregister_without_register();
        print!("Success.\n");
    }

    println!("All tests passed in poll. \n");
}

pub fn test_registering() {
    println!("Testing functions in registering inside enclave.\n");

    print!("Testing register_deregister()... ");
    registering::register_deregister();
    print!("Success.\n");

    print!("Testing reregister_different_interest_without_poll()... ");
    registering::reregister_different_interest_without_poll();
    print!("Success.\n");

    print!("Testing registering_after_deregistering()... ");
    registering::registering_after_deregistering();
    print!("Success.\n");

    // Insert debug_assertions tests here
    #[cfg(debug_assertions)]
    {
        print!("Testing tcp_register_multiple_event_loops()... ");
        registering::tcp_register_multiple_event_loops();
        print!("Success.\n");
    }

    #[cfg(debug_assertions)]
    {
    print!("Testing udp_register_multiple_event_loops()... ");
    registering::udp_register_multiple_event_loops();
    print!("Success.\n");
    }

    println!("All tests passed in registering. \n");

}

pub fn test_regressions() {
    println!("Testing functions in regressions inside enclave.\n");

    print!("Testing issue_776()... ");
    regressions::issue_776();
    print!("Success.\n");

    print!("Testing issue_1205()... ");
    regressions::issue_1205();
    print!("Success.\n");

    #[cfg(any(unix, target_env = "sgx"))]
    print!("Testing issue_1403()... ");
    regressions::issue_1403();
    print!("Success.\n");

    println!("All tests passed in regressions. \n");

}

pub fn test_tcp_listener() {
    println!("Testing functions in tcp_listener inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    tcp_listener::is_send_and_sync();
    print!("Success.\n");

    print!("Testing tcp_listener()... ");
    tcp_listener::tcp_listener();
    print!("Success.\n");

    print!("Testing tcp_listener_std()... ");
    tcp_listener::tcp_listener_std();
    print!("Success.\n");

    print!("Testing set_get_ttl()... ");
    tcp_listener::set_get_ttl();
    print!("Success.\n");

    print!("Testing get_ttl_without_previous_set()... ");
    tcp_listener::get_ttl_without_previous_set();
    print!("Success.\n");

    #[cfg(any(unix, target_env = "sgx"))]
    {
        print!("Testing raw_fd()... ");
        tcp_listener::raw_fd();
        print!("Success.\n");
    }

    print!("Testing registering()... ");
    tcp_listener::registering();
    print!("Success.\n");

    print!("Testing reregister()... ");
    tcp_listener::reregister();
    print!("Success.\n");

    print!("Testing no_events_after_deregister()... ");
    tcp_listener::no_events_after_deregister();
    print!("Success.\n");

    print!("Testing tcp_listener_two_streams()... ");
    tcp_listener::tcp_listener_two_streams();
    print!("Success.\n");

    // Note: ipv6 does not work on docker container.

    println!("All tests passed in tcp_listener. \n");

}

pub fn test_tcp_socket() {
    println!("Testing functions in tcp_socket inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    tcp_socket::is_send_and_sync();
    print!("Success.\n");

    print!("Testing set_reuseaddr()... ");
    tcp_socket::set_reuseaddr();
    print!("Success.\n");

    #[cfg(all(any(unix, target_env = "sgx"), not(any(target_os = "solaris", target_os = "illumos"))))]
    {
        print!("Testing set_reuseport()... ");
        tcp_socket::set_reuseport();
        print!("Success.\n");
    }

    print!("Testing set_keepalive()... ");
    tcp_socket::set_keepalive();
    print!("Success.\n");

    print!("Testing set_keepalive_time()... ");
    tcp_socket::set_keepalive_time();
    print!("Success.\n");

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "windows",
        target_env = "sgx"
    ))]
    {
        print!("Testing set_keepalive_interval()... ");
        tcp_socket::set_keepalive_interval();
        print!("Success.\n");
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_env = "sgx"
    ))]
    {
        print!("Testing set_keepalive_retries()... ");
        tcp_socket::set_keepalive_retries();
        print!("Success.\n");
    }

    print!("Testing get_localaddr()... ");
    tcp_socket::get_localaddr();
    print!("Success.\n");

    print!("Testing set_linger()... ");
    tcp_socket::set_linger();
    print!("Success.\n");

    print!("Testing send_buffer_size_roundtrips()... ");
    tcp_socket::send_buffer_size_roundtrips();
    print!("Success.\n");

    print!("Testing recv_buffer_size_roundtrips()... ");
    tcp_socket::recv_buffer_size_roundtrips();
    print!("Success.\n");

    println!("All tests passed in tcp_socket. \n");
}

pub fn test_tcp_stream() {
    println!("Testing functions in tcp_stream inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    tcp_stream::is_send_and_sync();
    print!("Success.\n");

    print!("Testing tcp_stream_ipv4()... ");
    tcp_stream::tcp_stream_ipv4();
    print!("Success.\n");

    //ipv6 does not work

    print!("Testing tcp_stream_std()... ");
    tcp_stream::tcp_stream_std();

    print!("Testing set_get_ttl()... ");
    tcp_stream::set_get_ttl();
    print!("Success.\n");

    print!("Testing get_ttl_without_previous_set()... ");
    tcp_stream::get_ttl_without_previous_set();
    print!("Success.\n");

    print!("Testing set_get_nodelay()... ");
    tcp_stream::set_get_nodelay();
    print!("Success.\n");

    print!("Testing get_nodelay_without_previous_set()... ");
    tcp_stream::get_nodelay_without_previous_set();
    print!("Success.\n");

    print!("Testing shutdown_read()... ");
    tcp_stream::shutdown_read();
    print!("Success.\n");

    print!("Testing shutdown_both()... ");
    tcp_stream::shutdown_both();
    print!("Success.\n");

    print!("Testing raw_fd()... ");
    tcp_stream::raw_fd();
    print!("Success.\n");

    print!("Testing registering()... ");
    tcp_stream::registering();
    print!("Success.\n");

    print!("Testing reregistering()... ");
    tcp_stream::reregistering();
    print!("Success.\n");

    print!("Testing no_events_after_deregister()... ");
    tcp_stream::no_events_after_deregister();
    print!("Success.\n");

    #[cfg(not(windows))]
    {
        print!("Testing tcp_shutdown_client_read_close_event()... ");
        tcp_stream::tcp_shutdown_client_read_close_event();
        print!("Success.\n");
    }

    
    print!("Testing tcp_reset_close_event()... ");
    tcp_stream::tcp_reset_close_event();
    print!("Success.\n");

    #[cfg(not(windows))]
    #[cfg(not(any(target_os = "illumos")))]
    {
        print!("Testing tcp_shutdown_client_both_close_event()... ");
        tcp_stream::tcp_shutdown_client_both_close_event();
        print!("Success.\n");
    }

    print!("Testing hup_event_on_disconnect()... ");
    tcp_stream::hup_event_on_disconnect();
    print!("Success.\n");

    println!("All tests passed in tcp_stream. \n");

}

pub fn test_tcp() {
    println!("Testing functions in test inside enclave.\n");

    #[cfg(all(any(unix, target_env = "sgx"), not(debug_assertions)))]
    {
        print!("Testing assert_size()... ");
        tcp::assert_size();
        print!("Success.\n");
    }

    print!("Testing is_send_and_sync()... ");
    tcp::is_send_and_sync();
    print!("Success.\n");

    print!("Testing accept()... ");
    tcp::accept();
    print!("Success.\n");

    print!("Testing connect()... ");
    tcp::connect();
    print!("Success.\n");

    print!("Testing read()... ");
    tcp::read();
    print!("Success.\n");

    print!("Testing peek()... ");
    tcp::peek();
    print!("Success.\n");

    print!("Testing write()... ");
    tcp::write();
    print!("Success.\n");

    print!("Testing connect_then_close()... ");
    tcp::connect_then_close();
    print!("Success.\n");

    print!("Testing listen_then_close()... ");
    tcp::listen_then_close();
    print!("Success.\n");

    print!("Testing bind_twice_bad()... ");
    tcp::bind_twice_bad();
    print!("Success.\n");

    print!("Testing multiple_writes_immediate_success()... ");
    tcp::multiple_writes_immediate_success();
    print!("Success.\n");

    print!("Testing connection_reset_by_peer()... ");
    tcp::connection_reset_by_peer();
    print!("Success.\n");

    print!("Testing connect_error()... ");
    tcp::connect_error();
    print!("Success.\n");

    print!("Testing write_error()... ");
    tcp::write_error();
    print!("Success.\n");

    #[cfg(feature = "untrusted_time")]
    {
        print!("Testing write_shutdown()... ");
        tcp::write_shutdown();
        print!("Success.\n");
    }

    print!("Testing local_addr_ready()... ");
    tcp::local_addr_ready();
    print!("Success.\n");

    print!("Testing write_then_drop()... ");
    tcp::write_then_drop();
    print!("Success.\n");

    print!("Testing write_then_deregister()... ");
    tcp::write_then_deregister();
    print!("Success.\n");

    print!("Testing tcp_no_events_after_deregister()... ");
    tcp::tcp_no_events_after_deregister();
    print!("Success.\n");

    println!("All tests passed in tcp. \n");
}

pub fn test_udp_socket() {
    println!("Testing functions in udp_socket inside enclave.\n");

    #[cfg(all(any(unix, target_env = "sgx"), not(debug_assertions)))]
    {
        print!("Testing assert_size()... ");
        udp_socket::assert_size();
        print!("Success.\n");
    }

    print!("Testing empty_datagram()... ");
    udp_socket::empty_datagram();
    print!("Success.\n");

    print!("Testing is_send_and_sync()... ");
    udp_socket::is_send_and_sync();
    print!("Success.\n");

    print!("Testing unconnected_udp_socket_ipv4()... ");
    udp_socket::unconnected_udp_socket_ipv4();
    print!("Success.\n");

    //ipv6 does not work

    print!("Testing unconnected_udp_socket_std()... ");
    udp_socket::unconnected_udp_socket_std();
    print!("Success.\n");

    print!("Testing set_get_ttl()... ");
    udp_socket::set_get_ttl();
    print!("Success.\n");

    print!("Testing get_ttl_without_previous_set()... ");
    udp_socket::get_ttl_without_previous_set();
    print!("Success.\n");

    print!("Testing set_get_broadcast()... ");
    udp_socket::set_get_broadcast();
    print!("Success.\n");

    print!("Testing get_broadcast_without_previous_set()... ");
    udp_socket::get_broadcast_without_previous_set();
    print!("Success.\n");

    print!("Testing set_get_multicast_loop_v4()... ");
    udp_socket::set_get_multicast_loop_v4();
    print!("Success.\n");

    print!("Testing get_multicast_loop_v4_without_previous_set()... ");
    udp_socket::get_multicast_loop_v4_without_previous_set();
    print!("Success.\n");

    print!("Testing set_get_multicast_ttl_v4()... ");
    udp_socket::set_get_multicast_ttl_v4();
    print!("Success.\n");

    print!("Testing get_multicast_ttl_v4_without_previous_set()... ");
    udp_socket::get_multicast_ttl_v4_without_previous_set();
    print!("Success.\n");

    print!("Testing connected_udp_socket_ipv4()... ");
    udp_socket::connected_udp_socket_ipv4();
    print!("Success.\n");

    print!("Testing connected_udp_socket_std()... ");
    udp_socket::connected_udp_socket_std();
    print!("Success.\n");

    print!("Testing reconnect_udp_socket_sending()... ");
    udp_socket::reconnect_udp_socket_sending();
    print!("Success.\n");

    print!("Testing reconnect_udp_socket_receiving()... ");
    udp_socket::reconnect_udp_socket_receiving();
    print!("Success.\n");

    print!("Testing unconnected_udp_socket_connected_methods()... ");
    udp_socket::unconnected_udp_socket_connected_methods();
    print!("Success.\n");

    /* print!("Testing connected_udp_socket_unconnected_methods()... ");
    udp_socket::connected_udp_socket_unconnected_methods();
    print!("Success.\n"); */
    //Panics at: unexpected OK result
    // Linux (and Android) and Windows actually allow `send_to` even if the
    // socket is connected.

    #[cfg(any(unix, target_env = "sgx"))]
    {
        print!("Testing udp_socket_raw_fd()... ");
        udp_socket::udp_socket_raw_fd();
        print!("Success.\n");
    }

    print!("Testing udp_socket_register()... ");
    udp_socket::udp_socket_register();
    print!("Success.\n");

    print!("Testing udp_socket_reregister()... ");
    udp_socket::udp_socket_reregister();
    print!("Success.\n");

    print!("Testing udp_socket_no_events_after_deregister()... ");
    udp_socket::udp_socket_no_events_after_deregister();
    print!("Success.\n");

    print!("Testing udp_socket()... ");
    udp_socket::udp_socket();
    print!("Success.\n");

    print!("Testing udp_socket_send_recv()... ");
    udp_socket::udp_socket_send_recv();
    print!("Success.\n");

    print!("Testing udp_socket_discard()... ");
    udp_socket::udp_socket_discard();
    print!("Success.\n");

    #[cfg(not(target_os = "android"))]
    {
        print!("Testing multicast()... ");
        udp_socket::multicast();
        print!("Success.\n");
    }

    print!("Testing et_behavior_recv()... ");
    udp_socket::et_behavior_recv();
    print!("Success.\n");

    print!("Testing et_behavior_recv_from()... ");
    udp_socket::et_behavior_recv_from();
    print!("Success.\n");

    println!("All tests passed in udp_socket. \n");

}

pub fn test_unix_datagram() {
    println!("Testing functions in unix_datagram inside enclave.\n");

    print!("Testing is_send_and_sync()... ");
    unix_datagram::is_send_and_sync();
    print!("Success.\n");
    
    print!("Testing unix_datagram_smoke_unconnected()... ");
    unix_datagram::unix_datagram_smoke_unconnected();
    print!("Success.\n");

    print!("Testing unix_datagram_smoke_connected()... ");
    unix_datagram::unix_datagram_smoke_connected();
    print!("Success.\n");

    print!("Testing unix_datagram_smoke_unconnected_from_std()... ");
    unix_datagram::unix_datagram_smoke_unconnected_from_std();
    print!("Success.\n");

    print!("Testing unix_datagram_smoke_connected_from_std()... ");
    unix_datagram::unix_datagram_smoke_connected_from_std();
    print!("Success.\n");

    print!("Testing unix_datagram_connect()... ");
    unix_datagram::unix_datagram_connect();
    print!("Success.\n");

    print!("Testing unix_datagram_pair()... ");
    unix_datagram::unix_datagram_pair();
    print!("Success.\n");
    
    print!("Testing unix_datagram_shutdown()... ");
    unix_datagram::unix_datagram_shutdown();
    print!("Success.\n");

    print!("Testing unix_datagram_register()... ");
    unix_datagram::unix_datagram_register();
    print!("Success.\n");

    print!("Testing unix_datagram_reregister()... ");
    unix_datagram::unix_datagram_reregister();
    print!("Success.\n");

    print!("Testing unix_datagram_deregister()... ");
    unix_datagram::unix_datagram_deregister();
    print!("Success.\n");

    println!("All tests passed in unix_datagram. \n");

}

pub fn test_unix_listener() {
    println!("Testing functions in unix_listener inside enclave.\n");

    print!("Testing unix_listener_send_and_sync()... ");
    unix_listener::unix_listener_send_and_sync();
    print!("Success.\n");

    print!("Testing unix_listener_smoke()... ");
    unix_listener::unix_listener_smoke();
    print!("Success.\n");

    print!("Testing unix_listener_from_std()... ");
    unix_listener::unix_listener_from_std();
    print!("Success.\n");

    print!("Testing unix_listener_local_addr()... ");
    unix_listener::unix_listener_local_addr();
    print!("Success.\n");

    print!("Testing unix_listener_register()... ");
    unix_listener::unix_listener_register();
    print!("Success.\n");

    print!("Testing unix_listener_reregister()... ");
    unix_listener::unix_listener_reregister();
    print!("Success.\n");

    print!("Testing unix_listener_deregister()... ");
    unix_listener::unix_listener_deregister();
    print!("Success.\n");

    println!("All tests passed in unix_listener. \n");
    
}

pub fn test_unix_stream() {

    println!("Testing functions in unix_stream inside enclave.\n");

    print!("Testing unix_stream_send_and_sync()... ");
    unix_stream::unix_stream_send_and_sync();
    print!("Success.\n");

    print!("Testing unix_stream_smoke()... ");
    unix_stream::unix_stream_smoke();
    print!("Success.\n");

    print!("Testing unix_stream_connect()... ");
    unix_stream::unix_stream_connect();
    print!("Success.\n");

    print!("Testing unix_stream_from_std()... ");
    unix_stream::unix_stream_from_std();
    print!("Success.\n");

    print!("Testing unix_stream_pair()... ");
    unix_stream::unix_stream_pair();
    print!("Success.\n");

    print!("Testing unix_stream_peer_addr()... ");
    unix_stream::unix_stream_peer_addr();
    print!("Success.\n");

    print!("Testing unix_stream_shutdown_read()... ");
    unix_stream::unix_stream_shutdown_read();
    print!("Success.\n");

    print!("Testing unix_stream_shutdown_write()... ");
    unix_stream::unix_stream_shutdown_write();
    print!("Success.\n");

    print!("Testing unix_stream_shutdown_both()... ");
    unix_stream::unix_stream_shutdown_both();
    print!("Success.\n");

    print!("Testing unix_stream_shutdown_listener_write()... ");
    unix_stream::unix_stream_shutdown_listener_write();
    print!("Success.\n");

    print!("Testing unix_stream_register()... ");
    unix_stream::unix_stream_register();
    print!("Success.\n");

    print!("Testing unix_stream_reregister()... ");
    unix_stream::unix_stream_reregister();
    print!("Success.\n");

    print!("Testing unix_stream_deregisters()... ");
    unix_stream::unix_stream_deregister();
    print!("Success.\n");

    println!("All tests passed in unix_stream. \n");

}