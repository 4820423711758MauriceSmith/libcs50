public class CS50 {
    // 加载cs50库
    static {
        System.loadLibrary("cs50");
    }
    
    // 原生方法声明
    public native char getChar(String prompt) throws EOFException;
    public native double getDouble(String prompt) throws EOFException;
    public native float getFloat(String prompt) throws EOFException;
    public native int getInt(String prompt) throws EOFException;
    public native long getLong(String prompt) throws EOFException;
    public native String getString(String prompt) throws EOFException;
    
    // 定义EOF异常
    public static class EOFException extends Exception {
        public EOFException() {
            super("输入错误或EOF");
        }
        
        public EOFException(String message) {
            super(message);
        }
    }
    
    // 测试方法
    public static void main(String[] args) {
        CS50 cs50 = new CS50();
        try {
            String name = cs50.getString("What is your name? ");
            System.out.println("Hello, " + name + "!");
            
            int age = cs50.getInt("How old are you? ");
            System.out.println("You are " + age + " years old.");
        } catch (EOFException e) {
            System.err.println(e.getMessage());
        }
    }
}