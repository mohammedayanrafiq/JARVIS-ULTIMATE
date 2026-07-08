$path = "c:\Users\Admin\OneDrive\Desktop\jarvis-desktop\src\index.html"
$html = [System.IO.File]::ReadAllText($path, [System.Text.Encoding]::UTF8)

# 1. Remove Firebase scripts from head
$firebase_scripts = @"
<script src="https://www.gstatic.com/firebasejs/10.8.0/firebase-app-compat.js"></script>
<script src="https://www.gstatic.com/firebasejs/10.8.0/firebase-auth-compat.js"></script>
"@
$html = $html.Replace($firebase_scripts, "")

# 2. Remove auth overlay from body
$auth_overlay = @"
<div id="tauri-auth-overlay" style="position:fixed;inset:0;z-index:999999;background:#080810;display:flex;flex-direction:column;align-items:center;justify-content:center;">
    <h1 style="color:#c8a97e;font-family:'Instrument Serif', serif;font-size:40px;margin-bottom:20px;font-style:italic;">JARVIS OS</h1>
    <p style="color:#e8e8f0;font-family:'DM Sans', sans-serif;margin-bottom:30px;">Authentication Required</p>
    <button onclick="window.loginGoogle()" style="padding:12px 24px;background:#c8a97e;color:#000;border-radius:8px;font-size:16px;margin-bottom:10px;font-weight:600;border:none;cursor:pointer;">Sign in with Google</button>
</div>
"@
$html = $html.Replace($auth_overlay, "")
$html = $html.Replace($auth_overlay + "`r`n", "")
$html = $html.Replace($auth_overlay + "`n", "")

# 3. Remove auth logic from script
$auth_logic = @"
const firebaseConfig = {
    apiKey: "YOUR_API_KEY",
    authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
    projectId: "YOUR_PROJECT_ID"
};
firebase.initializeApp(firebaseConfig);
const auth = firebase.auth();

window.loginGoogle = function() {
    const provider = new firebase.auth.GoogleAuthProvider();
    auth.signInWithPopup(provider).catch(e => alert(e.message));
}

auth.onAuthStateChanged(user => {
    if (user) {
        document.getElementById('tauri-auth-overlay').style.display = 'none';
        console.log("Logged in as", user.email);
    } else {
        document.getElementById('tauri-auth-overlay').style.display = 'flex';
    }
});
"@
$html = $html.Replace($auth_logic, "")
$html = $html.Replace($auth_logic + "`r`n", "")
$html = $html.Replace($auth_logic + "`n", "")

[System.IO.File]::WriteAllText($path, $html, [System.Text.Encoding]::UTF8)
Write-Host "Successfully removed Firebase from index.html"
