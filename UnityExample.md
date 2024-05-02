# Example of implementation in unity

'''cs
using System.Diagnostics;
using System.Runtime.InteropServices;
using UnityEngine.Networking;

namespace Game.Core {
    public static class LogServer
    {
        public static void Log(string msg, string url = "http://127.0.0.1:8000") {
            #if UNITY_EDITOR
                UnityWebRequest.Get(url + "/log/unity/debug/" + msg).SendWebRequest();
            #endif
        }

        public static void LogError(string msg, string url = "http://127.0.0.1:8000") {
            #if UNITY_EDITOR
                UnityWebRequest.Get(url + "/log/unity/error/" + msg).SendWebRequest();
            #endif
        }

        public static void LogWarning(string msg, string url = "http://127.0.0.1:8000") {
            #if UNITY_EDITOR
                UnityWebRequest.Get(url + "/log/unity/warning/" + msg).SendWebRequest();
            #endif
        }
        
        public static void Watch(string name, string value, string url = "http://127.0.0.1:8000") {
            #if UNITY_EDITOR
                UnityWebRequest.Get(url + "/watch/" + name + "/" + value).SendWebRequest();
            #endif
        }
    }
}
'''
