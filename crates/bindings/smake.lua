function smake.build()
    smake.macos()
    smake.windows()
end

function smake.windows()
    smake.clean()
    run('cargo build --target x86_64-pc-windows-gnu --features "rebuild-bindings"')
end

function smake.macos()
    smake.clean()
    run('cargo build --target x86_64-apple-darwin --features "rebuild-bindings"')
end

function smake.clean()
    run('cargo clean');
end
