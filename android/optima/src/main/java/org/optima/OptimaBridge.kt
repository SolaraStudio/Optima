package org.optima

class OptimaBridge(private val engine: OptimaEngine) {

    private val kotlinHandlers = mutableMapOf<String, (String) -> String>()

    fun registerKotlinHandler(name: String, handler: (String) -> String) {
        kotlinHandlers[name] = handler
    }

    fun callKotlinHandler(name: String, arg: String): String? {
        return kotlinHandlers[name]?.invoke(arg)
    }

    fun callNativeHandler(name: String, arg: String): String {
        return engine.callNativeHandler(name, arg)
    }

    fun hasKotlinHandler(name: String): Boolean {
        return kotlinHandlers.containsKey(name)
    }

    fun hasNativeHandler(name: String): Boolean {
        return engine.hasNativeHandler(name)
    }

    fun hasHandler(name: String): Boolean {
        return hasKotlinHandler(name) || hasNativeHandler(name)
    }

    fun allHandlerNames(): List<String> {
        val set = mutableSetOf<String>()
        set.addAll(kotlinHandlers.keys)
        set.addAll(engine.handlerNames())
        return set.sorted()
    }

    fun callAnyHandler(name: String, arg: String): String {
        if (hasKotlinHandler(name)) {
            return callKotlinHandler(name, arg) ?: ""
        }
        return callNativeHandler(name, arg)
    }
}
