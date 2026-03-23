[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

$rootDir = Resolve-Path (Join-Path $PSScriptRoot "..")
$wasmDir = Join-Path $rootDir "apps/wasm"

Push-Location $rootDir
try
{
    & wasm-pack build --release --target web apps/wasm
}
finally
{
    Pop-Location
}

$port = 8000
$prefix = "http://127.0.0.1:$port/"
$listener = [System.Net.HttpListener]::new()
$listener.Prefixes.Add($prefix)
$listener.Start()

Start-Process "$prefix`index.html"
Write-Host "Serving $wasmDir on $prefix`index.html"

$contentTypes = @{
    ".html" = "text/html"
    ".js" = "application/javascript"
    ".wasm" = "application/wasm"
    ".css" = "text/css"
    ".svg" = "image/svg+xml"
    ".json" = "application/json"
    ".map" = "application/json"
}

while ($listener.IsListening)
{
    $context = $listener.GetContext()
    try
    {
        $relativePath = $context.Request.Url.AbsolutePath.TrimStart("/")
        if ([string]::IsNullOrWhiteSpace($relativePath))
        {
            $relativePath = "index.html"
        }

        $filePath = Join-Path $wasmDir $relativePath
        if (-not (Test-Path $filePath -PathType Leaf))
        {
            $context.Response.StatusCode = 404
            $context.Response.Close()
            continue
        }

        $extension = [System.IO.Path]::GetExtension($filePath)
        $contentType = $contentTypes[$extension]
        if (-not $contentType)
        {
            $contentType = "application/octet-stream"
        }

        $bytes = [System.IO.File]::ReadAllBytes($filePath)
        $context.Response.ContentType = $contentType
        $context.Response.ContentLength64 = $bytes.Length
        $context.Response.OutputStream.Write($bytes, 0, $bytes.Length)
        $context.Response.Close()
    }
    catch
    {
        $context.Response.StatusCode = 500
        $context.Response.Close()
    }
}
