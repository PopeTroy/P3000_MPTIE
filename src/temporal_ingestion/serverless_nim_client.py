import os
import requests

class ServerlessNIMEngine:
    def __init__(self):
        self.endpoint = "https://integrate.api.nvidia.com/v1/chat/completions"
        self.api_key = os.environ.get("NGC_API_KEY", "")

    def invoke_inference(self, model_name: str, prompt: str):
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json"
        }
        payload = {
            "model": model_name,  # Using active model IDs like nvidia/nemotron-3.5-lightning-30b-a3b
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 100,
            "temperature": 0.3
        }

        response = requests.post(self.endpoint, headers=headers, json=payload)
        if response.status_code == 200:
            return response.json()['choices'][0]['message']['content']
        else:
            raise RuntimeError(f"NVCF Serverless Error [{response.status_code}]: {response.text}")
