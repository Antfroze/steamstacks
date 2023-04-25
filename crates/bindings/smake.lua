function smake.macos()
    clean()
    run('cargo build --target x86_64-apple-darwin --features "rebuild-bindings"')
end

function clean()
    run('cargo clean');
end