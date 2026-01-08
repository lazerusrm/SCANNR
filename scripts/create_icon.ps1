# PowerShell script to create a Windows icon from the RustScan logo
# Requires ImageMagick (magick command) or provides instructions

$iconPath = "assets\rustscan.ico"
$sourceImage = "pictures\rustscan.png"

if (Test-Path $sourceImage) {
    Write-Host "Found source image: $sourceImage" -ForegroundColor Green
    
    # Check if ImageMagick is available
    $magickAvailable = $false
    try {
        $null = Get-Command magick -ErrorAction Stop
        $magickAvailable = $true
    } catch {
        Write-Host "ImageMagick not found. Trying alternative methods..." -ForegroundColor Yellow
    }
    
    if ($magickAvailable) {
        Write-Host "Creating icon with ImageMagick..." -ForegroundColor Green
        magick convert $sourceImage -define icon:auto-resize=256,128,64,48,32,16 $iconPath
        if (Test-Path $iconPath) {
            Write-Host "Icon created successfully at: $iconPath" -ForegroundColor Green
            Write-Host "Rebuild with: cargo build --release --bin rustscan-gui" -ForegroundColor Cyan
        }
    } else {
        Write-Host "`nImageMagick not available. Please use one of these methods:" -ForegroundColor Yellow
        Write-Host "1. Install ImageMagick: https://imagemagick.org/script/download.php" -ForegroundColor Cyan
        Write-Host "2. Use online converter: https://convertio.co/png-ico/" -ForegroundColor Cyan
        Write-Host "3. Use GIMP or Photoshop to export as ICO" -ForegroundColor Cyan
        Write-Host "`nSource image location: $sourceImage" -ForegroundColor White
        Write-Host "Target icon location: $iconPath" -ForegroundColor White
    }
} else {
    Write-Host "Source image not found: $sourceImage" -ForegroundColor Red
    Write-Host "Please ensure the RustScan logo PNG exists in the pictures directory." -ForegroundColor Yellow
}

