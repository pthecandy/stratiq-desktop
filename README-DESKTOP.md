# StratIQ for Windows (Tauri)

A lightweight Windows app that opens StratIQ in its own window.

- **Your Riot keys stay safe.** The API key and sign-in secret stay on your server, never in the app.
- **No access to your PC.** The page inside the app can't touch your files or programs.
- **Stays in its lane.** Only stratiq.win and Riot's sign-in open inside the window; every other link opens in your normal browser.
- **Never touches the game.** It runs nothing while you play, and nothing is left running after you close it.
- **One window.** Opening it again brings the open window to the front.

GitHub builds the installer for you. You don't need to install Rust or anything else.

## 1. Make a new repository for it
github.com → **New repository** → name it `stratiq-desktop`.

- Use a **separate** repo, not `stratiq`. Railway deploys `stratiq`, and these files would confuse it.
- **Public** is best. There's nothing secret in these files, builds are free and unlimited, and anyone can download the installer. **Private** also works, but builds count against GitHub's monthly free minutes.

## 2. Upload the files
Click **uploading an existing file** and drag in:
`main.rs`, `Cargo.toml`, `build.rs`, `tauri.conf.json`, `offline.html`, `app-icon.png`, `README-DESKTOP.md`

Click **Commit changes**. It's fine that they all land in one folder; the build puts them in place.

## 3. Add the build file (this starts the first build)
1. In the repo, click **Add file** → **Create new file**.
2. In the name box, type exactly: `.github/workflows/build-windows.yml`
   (typing the `/` characters creates the folders).
3. Open `build-windows.yml` from the zip in Notepad, copy everything, and paste it into the big box.
4. Click **Commit changes**.

## 4. Get the installer
1. Open the repo's **Actions** tab. "Build StratIQ for Windows" is running; the first build takes about 15–25 minutes, later ones are faster.
2. When it shows a green check, go to the repo's **Releases** (right side of the repo's main page) → the newest one → download `StratIQ_1.0.0_x64-setup.exe`.
   It's also under the finished run → **Artifacts**.
3. Run it. Windows may show **"Windows protected your PC"**, because the app isn't code-signed yet. Click **More info** → **Run anyway**.

If the build fails, open the failed run, click the red step, and send the error to Claude.

## Updating
- **Website changes** (new features, fixes): nothing to do. The app shows stratiq.win, so it's always current.
- **Changing the app itself** (icon, window, name): upload the changed file, and GitHub builds a new installer automatically. You can also start one by hand: **Actions** → **Build StratIQ for Windows** → **Run workflow**.

## Before it works
- stratiq.win has to be live, because the app opens `https://stratiq.win/app`.
- Upload the new `server.js` to your **stratiq** repo (the website one) so `/app` works.
