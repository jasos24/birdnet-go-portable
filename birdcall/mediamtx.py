import requests
from .config import MEDIAMTX_URL

def is_mediamtx_alive():
    try:
        r = requests.get(f"{MEDIAMTX_URL}/v1/config/get", timeout=2)
        return r.status_code == 200
    except:
        return False
