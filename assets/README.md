# RustScan Icon

Place a Windows icon file (`rustscan.ico`) in this directory to have it embedded in the Windows executable.

## Creating an Icon

You can create an icon file using:

1. **Online Tools:**
   - https://convertio.co/png-ico/
   - https://www.icoconverter.com/
   - Convert any PNG/JPG image to ICO format

2. **Recommended Icon Sizes:**
   - 16x16, 32x32, 48x48, 64x64, 128x128, 256x256 pixels
   - Most tools will generate all sizes automatically

3. **Using ImageMagick (if installed):**
   ```powershell
   magick convert rustscan.png -define icon:auto-resize=256,128,64,48,32,16 rustscan.ico
   ```

4. **Using GIMP or Photoshop:**
   - Export as ICO format with multiple sizes

## Default Behavior

If no icon is found, the executable will still build and run, but will use the default Windows application icon.

